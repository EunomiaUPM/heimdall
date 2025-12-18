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

use anyhow::bail;
use async_trait::async_trait;
use tracing::{error, info};

use crate::errors::Errors;
use crate::services::gatekeeper::GateKeeperTrait;
use crate::services::issuer::IssuerTrait;
use crate::services::repo::RepoTrait;
use crate::services::vcs_builder::VcBuilderTrait;
use crate::services::verifier::VerifierTrait;
use crate::types::enums::errors::BadFormat;
use crate::types::enums::vc_type::VcType;
use crate::types::gnap::{GrantRequest, GrantResponse, RefBody};

#[async_trait]
pub trait CoreGatekeeperTrait: Send + Sync + 'static {
    fn gatekeeper(&self) -> Arc<dyn GateKeeperTrait>;
    fn verifier(&self) -> Arc<dyn VerifierTrait>;
    fn issuer(&self) -> Arc<dyn IssuerTrait>;
    fn repo(&self) -> Arc<dyn RepoTrait>;
    fn vc_builder(&self) -> Arc<dyn VcBuilderTrait>;
    async fn manage_req(&self, payload: GrantRequest) -> anyhow::Result<GrantResponse> {
        let (n_req_mod, n_int_model) = self.gatekeeper().start(payload)?;

        let req_model = self.repo().request().create(n_req_mod).await?;
        let int_model = self.repo().interaction().create(n_int_model).await?;

        let iss_model = self.issuer().start_vci(&req_model);

        let _iss_model = self.repo().issuing().create(iss_model).await?;

        if int_model.start.contains(&"oidc4vp".to_string()) {
            let n_ver_model = self.verifier().start_vp(&int_model.id)?;

            let ver_model = self.repo().verification().create(n_ver_model).await?;

            let uri = self.verifier().generate_verification_uri(ver_model);

            let response = GrantResponse::default4oidc4vp(
                int_model.id,
                int_model.continue_endpoint,
                int_model.continue_token,
                int_model.as_nonce,
                uri
            );
            return Ok(response);
        }
        if int_model.start.contains(&"cross-user".to_string()) {
            return self.gatekeeper().manage_cross_user(int_model);
        }
        let error = Errors::format_new(BadFormat::Received, "Interact method not supported");
        error!("{}", error);
        bail!(error)
    }
    async fn manage_cont_req(
        &self,
        cont_id: String,
        payload: RefBody,
        token: String
    ) -> anyhow::Result<String> {
        let int_model = self.repo().interaction().get_by_cont_id(&cont_id).await?;
        let mut iss_model = self.repo().issuing().get_by_id(&int_model.id).await?;
        let mut req_model = self.repo().request().get_by_id(&int_model.id).await?;

        let vc_type = VcType::from_str(&req_model.vc_type)?;

        self.gatekeeper().validate_cont_req(&int_model, payload.interact_ref, token)?;
        self.gatekeeper().validate_vc_to_issue(&vc_type)?;

        let credential_data = self.vc_builder().gather_data(&req_model)?;
        let vc_uri = self.issuer().generate_issuing_uri(&int_model.id);
        info!(vc_uri);

        req_model.status = "Approved".to_string();
        req_model.vc_uri = Some(vc_uri.clone());
        iss_model.uri = Some(vc_uri.clone());
        iss_model.credential_data = Some(credential_data);

        let _req_model = self.repo().request().update(req_model).await?;
        let _iss_model = self.repo().issuing().update(iss_model).await?;

        Ok(vc_uri)
    }
}
