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

use crate::services::{HasGateKeeper, HasNotifier, HasRepo, HasVcBuilder};
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
    async fn inner_manage_grant_req(
        &self,
        payload: &Bytes,
        headers: &HeaderMap,
    ) -> Outcome<GrantResponse> {
        let grant_request = self.gatekeeper().validate_grant(&payload, &headers)?;

        let mut grant = self
            .gatekeeper()
            .build_grant_plan(grant_request.client.class_id.clone())?;
        let available_vcs = self.vc_builder().get_role().available_credentials();

        let issuance = self.issuer().build_issuance_plan(
            &grant.id,
            grant_request.kind,
            grant_request.client.clone(),
            &available_vcs,
        ).await?;

        grant.vc_type_config = Some(issuance.vc_type_config.clone());

        let mut grant = self.repo().recv_grant().create(grant).await?;
        let issuance = self.repo().issuance().create(issuance).await?;
        self.frontend_notifier().notify(&grant);

        let response = self
            .gatekeeper()
            .manage_grant(grant_request.interact.as_ref());

        match response {
            GrantResponseManager::Approved => {
                grant.status = GrantStatus::Approved;
                let grant = self.repo().recv_grant().update(grant).await?;
                
                let interaction = self.gatekeeper().build_interaction_plan(
                    &grant.id,
                    grant_request.interact,
                    grant_request.client,
                )?;

                self.repo().recv_interaction().create(interaction).await?;

                let cred_offer = self.issuer().get_cred_offer_data(&issuance);
                let vc_transmission_offer = VcTransmissionOffer::ByValue(cred_offer);
                let uri = self.issuer().generate_issuing_uri(vc_transmission_offer)?;

                Ok(GrantResponse::vc_approved(uri, issuance.vc_type_config))
            }
            GrantResponseManager::Pending => {
                grant.status = GrantStatus::Pending;
                let grant = self.repo().recv_grant().update(grant).await?;
                let interaction = self.gatekeeper().build_interaction_plan(
                    &grant.id,
                    grant_request.interact,
                    grant_request.client,
                )?;
                let verification = self.verifier().build_vp_plan(&grant.id)?;

                let ver_model = self.repo().recv_verification().create(verification).await?;
                let interaction = self.repo().recv_interaction().create(interaction).await?;

                let uri = self.verifier().generate_verification_uri(&ver_model);
                Ok(GrantResponse::pending(uri, &interaction))
            }
            GrantResponseManager::Processing => {
                grant.status = GrantStatus::Processing;
                let grant = self.repo().recv_grant().update(grant).await?;
                let interaction = self.gatekeeper().build_interaction_plan(
                    &grant.id,
                    grant_request.interact,
                    grant_request.client,
                )?;
                let interaction = self.repo().recv_interaction().create(interaction).await?;
                Ok(GrantResponse::processing(&interaction))
            }
        }
    }

    async fn inner_manage_cont_req(
        &self,
        cont_id: &str,
        payload: &Bytes,
        headers: &HeaderMap,
    ) -> Outcome<GrantResponse> {
        let int_model = self
            .repo()
            .recv_interaction()
            .get_by_cont_id(&cont_id)
            .await?;

        self.gatekeeper()
            .validate_cont_req(&int_model, &payload, &headers)?;

        let issuance = self.repo().issuance().get_by_id(&int_model.id).await?;
        let mut grant = self.repo().recv_grant().get_by_id(&int_model.id).await?;

        grant.status = GrantStatus::Approved;
        let _grant = self.repo().recv_grant().update(grant).await?;

        let cred_offer = self.issuer().get_cred_offer_data(&issuance);
        let vc_transmission_offer = VcTransmissionOffer::ByValue(cred_offer);
        let uri = self.issuer().generate_issuing_uri(vc_transmission_offer)?;

        Ok(GrantResponse::vc_approved(uri, issuance.vc_type_config))
    }
}
