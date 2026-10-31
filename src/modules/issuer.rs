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

use crate::services::{HasEvents, HasGateKeeper, HasRepo, HasVcBuilder};
use async_trait::async_trait;
use chrono::Utc;
use ymir::errors::{Errors, Outcome};
use ymir::services::{HasIssuer, HasWallet};
use ymir::types::gnap::GrantStatus;
use ymir::types::issuance::{
    AuthServerMetadata, CredentialRequest, GiveVC, IssuerMetadata, IssuingToken, OidcGrantType,
    TokenRequest, VcBody, VcCredOffer,
};
use ymir::utils::require_field;

#[async_trait]
pub trait IssuerModule:
    HasGateKeeper + HasIssuer + HasRepo + HasVcBuilder + HasWallet + HasEvents + Send + Sync + 'static
{
    #[tracing::instrument(level = "info", skip_all, err)]
    async fn get_cred_offer_data(&self, id: &str) -> Outcome<VcCredOffer> {
        let model = self.repo().issuance().get_by_id(&id).await?;
        Ok(self.issuer().get_cred_offer_data(&model))
    }
    #[tracing::instrument(level = "info", skip_all)]
    fn issuer_metadata(&self) -> IssuerMetadata {
        let vcs = self.vc_builder().get_role().available_credentials();
        self.issuer().get_issuer_metadata(&vcs)
    }

    #[tracing::instrument(level = "info", skip_all)]
    fn oauth_server_metadata(&self) -> AuthServerMetadata {
        self.issuer().get_oauth_server_data()
    }

    // async fn jwks(&self) -> Outcome<Value> {
    //     self.issuer().get_jwks_data().await
    // }

    #[tracing::instrument(level = "info", skip_all, err)]
    async fn get_token(&self, payload: TokenRequest) -> Outcome<IssuingToken> {
        match payload.grant_type {
            OidcGrantType::PreAuthorizedCode => {
                let model = self
                    .repo()
                    .issuance()
                    .get_by_pre_auth_code(&payload.pre_authorized_code)
                    .await
                    .map_err(|_| Errors::forbidden("pre_auth_code does not match", None))?;
                Ok(self.issuer().get_token(&model))
            }
            OidcGrantType::AuthorizationCode
            | OidcGrantType::RefreshToken
            | OidcGrantType::ClientCredentials
            | OidcGrantType::Other(_) => Err(Errors::not_impl("unsupported grant type", None)),
        }
    }

    #[tracing::instrument(level = "info", skip_all, err)]
    async fn get_credential(&self, payload: CredentialRequest, token: String) -> Outcome<GiveVC> {
        let mut issuance = self.repo().issuance().get_by_token(&token).await?;

        let (holder_did, vc_config) = self
            .issuer()
            .validate_cred_req(&issuance, payload, &token)
            .await?;

        issuance.build_ctx.holder_did = Some(holder_did.id().to_string());

        let claims = self.vc_builder().build_vc(&issuance, vc_config)?;
        let vc_jwt = self.issuer().sign_claims(&claims).await?.as_str().to_string();

        issuance.credential = Some(vc_jwt.clone());

        let issuance = self.repo().issuance().update(issuance).await?;

        let mut grant = self.repo().recv_grant().get_by_id(&issuance.id).await?;
        self.events().issuance(&grant, &issuance, "edit").await;
        let interaction = self
            .repo()
            .recv_interaction()
            .get_by_id(&issuance.id)
            .await?;

        let holder = require_field(issuance.build_ctx.holder_did.as_ref(), "holder did")?;
        let minion = self.gatekeeper().build_minion_plan(
            holder,
            &grant.participant_nick,
            &interaction.callback_uri,
        );
        let known = self.repo().participant().get_by_id(holder).await.is_ok();
        let participant = self.repo().participant().force_update(minion).await?;
        let action = if known { "edit" } else { "create" };
        self.events().participant(&grant, &participant, action).await;

        grant.status = GrantStatus::Finalized;
        grant.ended_at = Some(Utc::now());
        let grant = self.repo().recv_grant().update(grant).await?;
        self.events().grant(&grant, "edit").await;

        Ok(GiveVC::synchronous(vec![VcBody::jwt(vc_jwt)]))
    }
}
