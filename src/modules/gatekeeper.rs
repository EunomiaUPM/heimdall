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

use crate::services::{HasConfig, HasEvents, HasGateKeeper, HasRepo, HasVcBuilder};
use crate::types::GrantResponseManager;
use async_trait::async_trait;
use axum::body::Bytes;
use axum::http::HeaderMap;
use ymir::errors::Outcome;
use ymir::services::{HasIssuer, HasVerifier};
use ymir::types::gnap::grant_response::{ErrorResponse, GrantResponse};
use ymir::types::gnap::GrantStatus;
use ymir::types::issuance::VcTransmissionOffer;
use ymir::utils::errors_to_error_code;

#[async_trait]
pub trait GatekeeperModule:
    HasGateKeeper
    + HasVerifier
    + HasIssuer
    + HasRepo
    + HasVcBuilder
    + HasEvents
    + HasConfig
    + Send
    + Sync
    + 'static
{
    #[tracing::instrument(level = "info", skip_all)]
    async fn manage_grant_req(&self, payload: Bytes, headers: HeaderMap) -> GrantResponse {
        self.inner_manage_grant_req(&payload, &headers)
            .await
            .unwrap_or_else(|e| {
                e.log();
                let code = errors_to_error_code(&e);
                GrantResponse::Error(ErrorResponse { error: code })
            })
    }

    #[tracing::instrument(level = "info", skip_all)]
    async fn manage_cont_req(
        &self,
        cont_id: String,
        payload: Bytes,
        headers: HeaderMap,
    ) -> GrantResponse {
        self.inner_manage_cont_req(&cont_id, &payload, &headers)
            .await
            .unwrap_or_else(|e| {
                e.log();
                let code = errors_to_error_code(&e);
                GrantResponse::Error(ErrorResponse { error: code })
            })
    }

    // ========================================= INTERNALS =========================================
    #[tracing::instrument(level = "info", skip_all, err)]
    async fn inner_manage_grant_req(
        &self,
        payload: &Bytes,
        headers: &HeaderMap,
    ) -> Outcome<GrantResponse> {
        let grant_request = self.gatekeeper().validate_grant(&payload, &headers)?;
        let response = self
            .gatekeeper()
            .manage_grant(grant_request.interact.as_ref())?;

        let mut grant = self
            .gatekeeper()
            .build_grant_plan(grant_request.client.class_id.clone())?;
        let available_vcs = self.vc_builder().get_role().available_credentials();

        let issuance = self.issuer().build_issuance_plan(
            &grant.role,
            &grant.id,
            grant_request.kind,
            grant_request.client.clone(),
            &available_vcs,
        ).await?;

        grant.vc_type_config = Some(issuance.vc_type_config.clone());

        let mut grant = self.repo().recv_grant().create(grant).await?;
        let issuance = self.repo().issuance().create(issuance).await?;
        self.events().grant(&grant, "create").await;
        self.events().issuance(&grant, &issuance, "create").await;

        match response {
            GrantResponseManager::Approved => {
                grant.status = GrantStatus::Approved;
                let grant = self.repo().recv_grant().update(grant).await?;
                self.events().grant(&grant, "edit").await;

                let interaction = self.gatekeeper().build_interaction_plan(
                    &grant.id,
                    grant_request.interact,
                    grant_request.client,
                )?;

                let interaction = self.repo().recv_interaction().create(interaction).await?;
                self.events().interaction(&grant, &interaction, "create").await;

                let cred_offer = self.issuer().get_cred_offer_data(&issuance);
                let vc_transmission_offer = VcTransmissionOffer::ByValue(cred_offer);
                let uri = self.issuer().generate_issuing_uri(vc_transmission_offer)?;

                Ok(GrantResponse::vc_approved(uri, issuance.vc_type_config))
            }
            GrantResponseManager::Pending => {
                grant.status = GrantStatus::Pending;
                let grant = self.repo().recv_grant().update(grant).await?;
                self.events().grant(&grant, "edit").await;
                let interaction = self.gatekeeper().build_interaction_plan(
                    &grant.id,
                    grant_request.interact,
                    grant_request.client,
                )?;
                let vcs = self.config().get_requested_vcs().to_vec();
                let verification = self.verifier().build_vp_plan(&grant.id, &vcs)?;

                let ver_model = self.repo().recv_verification().create(verification).await?;
                let interaction = self.repo().recv_interaction().create(interaction).await?;
                self.events().verification(&grant, &ver_model, "create").await;
                self.events().interaction(&grant, &interaction, "create").await;

                let uri = self.verifier().generate_verification_uri(&ver_model).await?;
                Ok(GrantResponse::pending(uri, &interaction))
            }
            GrantResponseManager::Processing => {
                grant.status = GrantStatus::Processing;
                let grant = self.repo().recv_grant().update(grant).await?;
                self.events().grant(&grant, "edit").await;
                let interaction = self.gatekeeper().build_interaction_plan(
                    &grant.id,
                    grant_request.interact,
                    grant_request.client,
                )?;
                let interaction = self.repo().recv_interaction().create(interaction).await?;
                self.events().interaction(&grant, &interaction, "create").await;
                Ok(GrantResponse::processing(&interaction))
            }
        }
    }

    #[tracing::instrument(level = "info", skip_all, err)]
    async fn inner_manage_cont_req(
        &self,
        cont_id: &str,
        payload: &Bytes,
        headers: &HeaderMap,
    ) -> Outcome<GrantResponse> {
        let int_model = self
            .repo()
            .recv_interaction()
            .get_by_continuation_id(&cont_id)
            .await?;

        self.gatekeeper()
            .validate_cont_req(&int_model, &payload, &headers)?;

        let issuance = self.repo().issuance().get_by_id(&int_model.grant_id).await?;
        let mut grant = self.repo().recv_grant().get_by_id(&int_model.grant_id).await?;

        grant.status = GrantStatus::Approved;
        let grant = self.repo().recv_grant().update(grant).await?;
        self.events().grant(&grant, "edit").await;

        let cred_offer = self.issuer().get_cred_offer_data(&issuance);
        let vc_transmission_offer = VcTransmissionOffer::ByValue(cred_offer);
        let uri = self.issuer().generate_issuing_uri(vc_transmission_offer)?;

        Ok(GrantResponse::vc_approved(uri, issuance.vc_type_config))
    }
}
