/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
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

use crate::core::traits::{HasGateKeeper, HasNotifier, HasRepo, HasVcBuilder};
use crate::services::gatekeeper::GateKeeperTrait;
use crate::services::notifications::NotificationsTrait;
use crate::services::repo::RepoTrait;
use crate::services::vcs_builder::VcBuilderTrait;
use async_trait::async_trait;
use axum::body::Bytes;
use axum::http::HeaderMap;
use tracing::info;
use ymir::errors::Outcome;
use ymir::modules::{HasIssuer, HasVerifier};
use ymir::services::issuer::IssuerTrait;
use ymir::services::verifier::VerifierTrait;
use ymir::types::gnap::grant_request::interact::{InteractRequest, InteractStart};
use ymir::types::gnap::grant_response::{ErrorResponse, GrantResponse};
use ymir::types::vcs::VcType;
use ymir::utils::errors_to_error_code;

#[async_trait]
pub trait GatekeeperModuleTrait:
HasGateKeeper
+ HasVerifier
+ HasIssuer
+ HasRepo
+ HasVcBuilder
+ HasNotifier
+ Send
+ Sync
+ 'static
{
    async fn manage_grant_req(&self, payload: Bytes, headers: HeaderMap) -> GrantResponse {
        self.inner_manage_grant_req(&payload, &headers)
            .await
            .unwrap_or_else(|e| {
                e.log();
                let code = errors_to_error_code(&e);
                GrantResponse::Error(ErrorResponse { error: code })
            })
    }
    async fn inner_manage_grant_req(
        &self,
        payload: &Bytes,
        headers: &HeaderMap,
    ) -> Outcome<GrantResponse> {
        let grant_request = self.gatekeeper().validate_grant(&payload, &headers)?;

        let kk = match grant_request.interact {
            Some(interact) if interact.start.contains(&InteractStart::Oid4VP) => {
                // caso Oid4VP
            }
            _ => {

            }
        };

        let req_model = self.repo().request().create(n_req_mod).await?;

        self.notifier().notify(&req_model);

        let int_model = self.repo().interaction().create(n_int_model).await?;

        let iss_model = self.issuer().start_vci(&req_model);

        let mut iss_model = self.repo().issuing().create(iss_model).await?;

        if int_model.start.contains(&InteractStart::Oid4VP.to_string()) {
            let n_ver_model = self.verifier().start_vp(&int_model.id)?;

            let ver_model = self.repo().verification().create(n_ver_model).await?;

            let uri = self.verifier().generate_verification_uri(&ver_model);

            let response = GrantResponse::pending(&InteractStart::Oid4VP, &int_model, Some(&uri));

            Ok(response)
        } else {
            if self.gatekeeper().auto_approve_cert() {
                let credential_data = self.vc_builder().gather_data(&req_model)?;
                let mut req_model = req_model;
                req_model.status = "Approved".to_string();
                iss_model.credential_data = Some(credential_data);
                let iss_model = self.repo().issuing().update(iss_model).await?;
                self.repo().request().update(req_model).await?;
                GrantResponse::vc_approved(&iss_model)
            } else {
                self.gatekeeper().manage_cert(&int_model)
            }
        }
    }
    async fn manage_ok_req(&self, payload: &Bytes, headers: &HeaderMap) -> Outcome<GrantResponse> {
        let (n_req_mod, n_int_model) = self.gatekeeper().start(payload, headers)?;

        let req_model = self.repo().request().create(n_req_mod).await?;

        if let Some(notifier) = self.notifier() {
            notifier.notify(&req_model);
        }

        let int_model = self.repo().interaction().create(n_int_model).await?;

        let iss_model = self.issuer().start_vci(&req_model);

        let mut iss_model = self.repo().issuing().create(iss_model).await?;

        if int_model
            .start
            .contains(&InteractStart::Oidc4VP.to_string())
        {
            let n_ver_model = self.verifier().start_vp(&int_model.id)?;

            let ver_model = self.repo().verification().create(n_ver_model).await?;

            let uri = self.verifier().generate_verification_uri(&ver_model);

            let response = GrantResponse::pending(&InteractStart::Oidc4VP, &int_model, Some(&uri));

            Ok(response)
        } else {
            if self.gatekeeper().auto_approve_cert() {
                let credential_data = self.vc_builder().gather_data(&req_model)?;
                let mut req_model = req_model;
                req_model.status = "Approved".to_string();
                iss_model.credential_data = Some(credential_data);
                let iss_model = self.repo().issuing().update(iss_model).await?;
                self.repo().request().update(req_model).await?;
                GrantResponse::vc_approved(&iss_model)
            } else {
                self.gatekeeper().manage_cert(&int_model)
            }
        }
    }
    async fn manage_cont_req(
        &self,
        cont_id: String,
        payload: Bytes,
        headers: HeaderMap,
    ) -> Outcome<CredentialResponse> {
        let int_model = self.repo().interaction().get_by_cont_id(&cont_id).await?;

        self.gatekeeper()
            .validate_cont_req(&int_model, &payload, &headers)?;

        let mut iss_model = self.repo().issuing().get_by_id(&int_model.id).await?;
        let mut req_model = self.repo().request().get_by_id(&int_model.id).await?;

        let vc_type = VcType::from_str(&req_model.vc_type)?;

        self.gatekeeper().validate_vc_to_issue(&vc_type)?;

        let credential_data = self.vc_builder().gather_data(&req_model)?;
        info!(iss_model.uri);

        req_model.vc_uri = Some(iss_model.uri.clone());
        iss_model.credential_data = Some(credential_data);

        let _req_model = self.repo().request().update(req_model).await?;
        let iss_model = self.repo().issuing().update(iss_model).await?;
        Ok(CredentialResponse {
            credential_uri: iss_model.uri.clone(),
            credential_type: vc_type.to_conf(),
        })
    }
}
