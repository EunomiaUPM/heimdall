/*
 * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use axum::body::Bytes;
use axum::http::HeaderMap;
use serde_json::Value;
use tracing::info;
use ymir::capabilities::HttpSig;
use ymir::config::traits::HostsConfigTrait;
use ymir::config::types::HostType;
use ymir::data::entities::{recv_interaction, vc_request};
use ymir::errors::{BadFormat, Errors, Outcome};
use ymir::services::client::ClientTrait;
use ymir::types::gnap::grant_request::{GrantRequest, Interact4GR, InteractStart, KeyProof};
use ymir::types::gnap::grant_response::GrantResponse;
use ymir::types::gnap::{ApprovedCallbackBody, RefBody, RejectedCallbackBody};
use ymir::types::http::Body;
use ymir::types::vcs::VcType;
use ymir::utils::{
    create_opaque_token, extract_gnap_token, json_headers, parse_from_slice, parse_to_value
};

use super::config::{GnapConfig, GnapConfigTrait};
use crate::config::role::RoleConfigTrait;
use crate::services::gatekeeper::GateKeeperTrait;

pub struct GnapService {
    config: GnapConfig,
    client: Arc<dyn ClientTrait>
}

impl GnapService {
    pub fn new(config: GnapConfig, client: Arc<dyn ClientTrait>) -> Self {
        GnapService { config, client }
    }
}

#[async_trait]
impl GateKeeperTrait for GnapService {
    fn start(
        &self,
        payload: &Bytes,
        headers: &HeaderMap
    ) -> Outcome<(vc_request::NewModel, recv_interaction::NewModel)> {
        info!("Managing vc request");

        let (payload, interact) = self.validate_acc_req(&payload, headers)?;
        let id = uuid::Uuid::new_v4().to_string();
        let client = &payload.client;
        let cert = client.key.cert.as_deref().ok_or_else(|| {
            Errors::format(
                BadFormat::Received,
                "Right now only petitions including a cert are accepted",
                None
            )
        })?;
        let participant_slug = payload.client.class_id.as_deref().ok_or_else(|| {
            Errors::format(BadFormat::Received, "Missing field class_id in the petition", None)
        })?;

        let vc_req = payload.credential_request.as_ref().ok_or_else(|| {
            Errors::format(
                BadFormat::Received,
                "Missing field credential_request in the grant_request",
                None
            )
        })?;

        let vc_type = vc_req.access.datatypes.as_ref().ok_or_else(|| {
            Errors::format(BadFormat::Received, "No field datatypes in the request", None)
        })?;

        let vc_type = vc_type
            .first()
            .ok_or_else(|| Errors::format(BadFormat::Received, "Datatypes are empty", None))?;

        let vc_type = VcType::from_conf(vc_type)?;
        self.validate_vc_to_issue(&vc_type)?;

        let mut start = interact.start;
        if !start.contains(&"".to_string()) && !start.contains(&"oidc4vp".to_string()) {
            start = vec!["".to_string()];
        }

        let new_request_model = vc_request::NewModel {
            id: id.clone(),
            participant_slug: participant_slug.to_string(),
            cert: cert.to_string(),
            vc_type: vc_type.to_string(),
            interact_method: start.clone()
        };

        let host_url = format!(
            "{}{}/gate",
            self.config.hosts().get_host(HostType::Http),
            self.config.get_api_path()
        );
        let continue_endpoint = format!("{}/continue", &host_url);
        let grant_endpoint = format!("{}/access", &host_url);
        let continue_token = create_opaque_token();

        let new_recv_interaction_model = recv_interaction::NewModel {
            id: id.clone(),
            start,
            method: interact.finish.method,
            uri: interact.finish.uri.ok_or_else(|| {
                Errors::format(BadFormat::Received, "Interact finish URI is missing", None)
            })?,
            cert: cert.to_string(),
            client_nonce: interact.finish.nonce,
            hash_method: interact.finish.hash_method,
            hints: interact.hints,
            grant_endpoint,
            continue_endpoint,
            continue_token
        };

        Ok((new_request_model, new_recv_interaction_model))
    }

    fn validate_acc_req(
        &self,
        payload: &Bytes,
        headers: &HeaderMap
    ) -> Outcome<(GrantRequest, Interact4GR)> {
        info!("Validating vc access request");

        let grant_request: GrantRequest = parse_from_slice(payload)?;

        match grant_request.client.key.cert.as_deref() {
            Some(cert) => {
                let proof = KeyProof::from_str(&grant_request.client.key.proof)?;
                match proof {
                    KeyProof::HttpSig => {}
                    method => {
                        return Err(Errors::not_impl(
                            format!("Right now we only accept httpsig, not {}", method),
                            None
                        ))
                    }
                }

                let grant_endpoint = format!(
                    "{}{}/gate/access",
                    self.config.get_host(HostType::Http),
                    self.config.get_api_path()
                );
                HttpSig::verify(headers, "POST", &grant_endpoint, payload, &cert)?;

                HttpSig::check_cert(&cert)?;
            }
            None => {
                if let Some(_) = grant_request.client.key.jwk.as_ref() {
                    return Err(Errors::not_impl(
                        "Cannot make this flow with jwk yet, try with cert",
                        None
                    ));
                }
                return Err(Errors::format(
                    BadFormat::Received,
                    "Client certificate has not arrived",
                    None
                ));
            }
        }

        let interact = grant_request.interact.as_ref().ok_or_else(|| {
            Errors::not_impl(
                "Only petitions with an 'interact field' are supported right now",
                None
            )
        })?;

        interact.finish.uri.as_ref().ok_or_else(|| {
            Errors::format(BadFormat::Received, "Interact method does not have an uri", None)
        })?;

        Ok((grant_request.clone(), interact.clone()))
    }

    fn validate_vc_to_issue(&self, vc_type: &VcType) -> Outcome<()> {
        info!("Validating that the requested vc can be issued");

        let role = self.config.get_role();
        let available_vcs = role.credentials();
        if available_vcs.contains(vc_type) {
            Ok(())
        } else {
            let available =
                available_vcs.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ");

            Err(Errors::unauthorized(
                format!("As a {} we can only issue {}", role, available),
                None
            ))
        }
    }

    fn validate_cont_req(
        &self,
        int_model: &recv_interaction::Model,
        payload: &Bytes,
        headers: &HeaderMap
    ) -> Outcome<()> {
        info!("Validating continue request");

        let ref_body: RefBody = parse_from_slice(payload)?;

        HttpSig::verify(
            headers,
            "POST",
            &int_model.continue_endpoint,
            payload,
            &int_model.cert
        )?;

        HttpSig::check_cert(&int_model.cert)?;

        if ref_body.interact_ref != int_model.interact_ref {
            return Err(Errors::security(
                format!(
                    "Interact reference '{}' does not match '{}'",
                    ref_body.interact_ref, int_model.interact_ref
                ),
                None
            ));
        }

        let token = extract_gnap_token(headers)?;
        if token != int_model.continue_token {
            return Err(Errors::security(
                format!("Token '{}' does not match '{}'", token, int_model.continue_token),
                None
            ));
        }

        Ok(())
    }
    async fn end_verification(&self, model: &recv_interaction::Model) -> Outcome<Option<String>> {
        info!("Ending verification");

        if model.method == "redirect" {
            let redirect_uri = format!(
                "{}?hash={}&interact_ref={}",
                model.uri, model.hash, model.interact_ref
            );
            Ok(Some(redirect_uri))
        } else if model.method == "push" {
            let url = &model.uri;

            let body = ApprovedCallbackBody {
                interact_ref: model.interact_ref.clone(),
                hash: model.hash.clone()
            };
            self.client.post(&url, Some(json_headers()), Body::json(&body)?).await?;

            Ok(None)
        } else {
            Err(Errors::not_impl(
                format!("Interact method {} not supported", model.method),
                None
            ))
        }
    }

    async fn apprv_dny_req(
        &self,
        approve: bool,
        req_model: &mut vc_request::Model,
        int_model: &recv_interaction::Model
    ) -> Outcome<Value> {
        match approve {
            true => {
                info!("Approving petition to obtain a VC");
                req_model.status = "Approved".to_string();
                let body = ApprovedCallbackBody {
                    interact_ref: int_model.interact_ref.clone(),
                    hash: int_model.hash.clone()
                };
                parse_to_value(&body)
            }
            false => {
                info!("Rejecting petition to obtain a VC");
                req_model.status = "Finalized".to_string();
                let body = RejectedCallbackBody { rejected: "Petition was rejected".to_string() };
                parse_to_value(&body)
            }
        }
    }

    async fn notify_minion(&self, int_model: &recv_interaction::Model, body: Value) -> Outcome<()> {
        let res = self.client.post(&int_model.uri, Some(json_headers()), Body::Json(body)).await?;

        if res.status().is_success() {
            info!("Minion received callback successfully");
            Ok(())
        } else {
            Err(Errors::consumer(
                &int_model.uri,
                "POST",
                Some(res.status()),
                "Minion did not receive callback successfully",
                None
            ))
        }
    }

    fn manage_cert(&self, model: &recv_interaction::Model) -> Outcome<GrantResponse> {
        info!("Managing cross-user request");
        if self.config.is_cert_allowed() {
            Ok(GrantResponse::pending(&InteractStart::Cert, model, None))
        } else {
            Err(Errors::unauthorized(
                "Not able to allow certification using a cert",
                None
            ))
        }
    }
}
