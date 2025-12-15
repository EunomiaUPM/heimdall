/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */
use super::config::{GnapConfig, GnapConfigTrait};
use crate::data::entities::interaction::Model;
use crate::data::entities::{interaction, request};
use crate::errors::{ErrorLogTrait, Errors};
use crate::services::client::ClientServiceTrait;
use crate::services::gatekeeper::GateKeeperTrait;
use crate::types::enums::errors::BadFormat;
use crate::types::enums::request::Body;
use crate::types::enums::role::AuthorityRole;
use crate::types::enums::vc_type::VcType;
use crate::types::gnap::{
    CallbackBody, GrantRequest, GrantResponse, Interact4GR, RejectedCallbackBody,
};
use crate::utils::create_opaque_token;
use anyhow::bail;
use async_trait::async_trait;
use axum::http::header::{ACCEPT, CONTENT_TYPE};
use axum::http::HeaderMap;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, info};

pub struct GnapService {
    config: GnapConfig,
    client: Arc<dyn ClientServiceTrait>,
}

impl GnapService {
    pub fn new(config: GnapConfig, client: Arc<dyn ClientServiceTrait>) -> Self {
        GnapService { config, client }
    }
}

#[async_trait]
impl GateKeeperTrait for GnapService {
    fn start(
        &self,
        payload: GrantRequest,
    ) -> anyhow::Result<(request::NewModel, interaction::NewModel)> {
        info!("Managing vc request");

        let interact = self.validate_acc_req(&payload)?;
        let id = uuid::Uuid::new_v4().to_string();
        let client = payload.client;
        let cert = client.key.cert;
        let participant_slug = client.class_id.unwrap_or("Slug".to_string());
        let vc_type = VcType::from_str(&payload.access_token.access.r#type)?;

        self.validate_vc_to_issue(&vc_type)?;

        let new_request_model = request::NewModel {
            id: id.clone(),
            participant_slug,
            cert,
            vc_type: vc_type.to_string(),
        };

        let host_url = format!(
            "{}{}/gate",
            self.config.get_host(),
            self.config.get_api_path()
        );
        let continue_endpoint = format!("{}/continue", &host_url);
        let grant_endpoint = format!("{}/access", &host_url);
        let continue_token = create_opaque_token();

        let new_interaction_model = interaction::NewModel {
            id: id.clone(),
            start: interact.start,
            method: interact.finish.method,
            uri: interact.finish.uri.unwrap(), // EXPECTED ALWAYS (Checked in validate_acc_req)
            client_nonce: interact.finish.nonce,
            hash_method: interact.finish.hash_method,
            hints: interact.hints,
            grant_endpoint,
            continue_endpoint,
            continue_token,
        };

        Ok((new_request_model, new_interaction_model))
    }

    fn validate_acc_req(&self, payload: &GrantRequest) -> anyhow::Result<Interact4GR> {
        info!("Validating vc access request");

        let interact = match &payload.interact {
            Some(model) => model,
            None => {
                let cause = "Only petitions with an 'interact field' are supported right now";
                let error = Errors::not_impl_new(cause, cause);
                error!("{}", error.log());
                bail!(error);
            }
        };

        let start = interact.start.clone();
        if !&start.contains(&"cross-user".to_string()) && !&start.contains(&"oidc4vp".to_string()) {
            let cause = "Interact method not supported yet";
            let error = Errors::not_impl_new(cause, cause);
            error!("{}", error.log());
            bail!(error);
        }

        match interact.finish.uri {
            Some(_) => {}
            None => {
                let error =
                    Errors::format_new(BadFormat::Received, "Interact method does not have an uri");
                error!("{}", error.log());
                bail!(error)
            }
        }

        Ok(interact.clone())
    }

    fn validate_vc_to_issue(&self, vc_type: &VcType) -> anyhow::Result<()> {
        info!("Validating that the requested vc can be issued");

        match self.config.get_role() {
            AuthorityRole::LegalAuthority => match vc_type {
                VcType::LegalRegistrationNumber(_) => {}
                _ => {
                    let error = Errors::unauthorized_new(
                        "As a legal authority we can only issue LegalRegistration numbers vcs",
                    );
                    error!("{}", error.log());
                    bail!(error)
                }
            },
            AuthorityRole::ClearingHouse => {
                // TODO
            }
            AuthorityRole::ClearingHouseProxy => {
                // TODO
            }
            AuthorityRole::DataSpaceAuthority => {
                // TODO
            }
        }
        Ok(())
    }

    fn validate_cont_req(
        &self,
        int_model: &interaction::Model,
        int_ref: String,
        token: String,
    ) -> anyhow::Result<()> {
        info!("Validating continue request");

        if int_ref != int_model.interact_ref {
            let error = Errors::security_new(&format!(
                "Interact reference '{}' does not match '{}'",
                int_ref, int_model.interact_ref
            ));
            error!("{}", error.log());
            bail!(error);
        }

        if token != int_model.continue_token {
            let error = Errors::security_new(&format!(
                "Token '{}' does not match '{}'",
                token, int_model.continue_token
            ));
            error!("{}", error.log());
            bail!(error);
        }
        Ok(())
    }
    async fn end_verification(&self, model: interaction::Model) -> anyhow::Result<Option<String>> {
        info!("Ending verification");

        if model.method == "redirect" {
            let redirect_uri = format!(
                "{}?hash={}&interact_ref={}",
                model.uri, model.hash, model.interact_ref
            );
            Ok(Some(redirect_uri))
        } else if model.method == "push" {
            let url = model.uri;

            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/json".parse()?);
            headers.insert(ACCEPT, "application/json".parse()?);

            let body = CallbackBody {
                interact_ref: model.interact_ref,
                hash: model.hash,
            };
            let body = serde_json::to_value(body)?;
            self.client
                .post(&url, Some(headers), Body::Json(body))
                .await?;

            Ok(None)
        } else {
            let error = Errors::not_impl_new(
                "Interact method not supported",
                &format!("Interact method {} not supported", model.method),
            );
            error!("{}", error.log());
            bail!(error);
        }
    }

    async fn apprv_dny_req(
        &self,
        approve: bool,
        req_model: &mut request::Model,
        int_model: &interaction::Model,
    ) -> anyhow::Result<()> {
        let body = match approve {
            true => {
                info!("Approving petition to obtain a VC");
                req_model.status = "Approved".to_string();
                let body = CallbackBody {
                    interact_ref: int_model.interact_ref.clone(),
                    hash: int_model.hash.clone(),
                };
                serde_json::to_value(body)?
            }
            false => {
                info!("Rejecting petition to obtain a VC");
                req_model.status = "Finalized".to_string();
                let body = RejectedCallbackBody {
                    rejected: "Petition was rejected".to_string(),
                };
                serde_json::to_value(body)?
            }
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        headers.insert(ACCEPT, "application/json".parse()?);

        let res = self
            .client
            .post(&int_model.uri, Some(headers), Body::Json(body))
            .await?;

        match res.status().as_u16() {
            200 => {
                info!("Minion received callback received successfully");
            }
            _ => {
                let error = Errors::consumer_new(
                    &int_model.uri,
                    "POST",
                    Some(res.status().as_u16()),
                    "Minion did not receive callback successfully",
                );
                req_model.status = "Minion_failure".to_string();
                error!("{}", error.log());
                bail!(error);
            }
        }

        Ok(())
    }

    fn manage_cross_user(&self, model: Model) -> anyhow::Result<GrantResponse> {
        match self.config.is_cert_allowed() {
            true => {
                let response = GrantResponse::default4cross_user(
                    model.id,
                    model.continue_endpoint,
                    model.continue_token,
                    model.as_nonce,
                );
                Ok(response)
            }
            false => {
                let error =
                    Errors::unauthorized_new("Not able to allow authorization using a cert");
                error!("{}", error.log());
                bail!(error)
            }
        }
    }
}
