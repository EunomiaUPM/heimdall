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

use crate::services::{HasGateKeeper, HasRepo};
use async_trait::async_trait;
use chrono::Utc;
use ymir::errors::{Errors, Outcome};
use ymir::services::HasVerifier;
use ymir::types::gnap::InteractionFinishResponse;
use ymir::types::vcs::VPDef;
use ymir::types::verification::{VerificationStatus, VerifyPayload};

#[async_trait]
pub trait VerifierModule: HasGateKeeper + HasVerifier + HasRepo + Send + Sync + 'static {
    async fn get_vp_def(&self, state: String) -> Outcome<VPDef> {
        let verification = self.repo().recv_verification().get_by_state(&state).await?;
        self.verifier().generate_vpd(&verification)
    }
    /// Signed Request Object of the verification opened with `state` (OpenID4VP 1.0
    /// `request_uri`), echoing the wallet's `wallet_nonce`, if any.
    async fn request_object(&self, state: String, wallet_nonce: Option<String>) -> Outcome<String> {
        let verification = self.repo().recv_verification().get_by_state(&state).await?;
        self.verifier().request_object(&verification, wallet_nonce).await
    }
    async fn verify(
        &self,
        state: String,
        payload: VerifyPayload,
    ) -> Outcome<InteractionFinishResponse> {
        let mut verification = self.repo().recv_verification().get_by_state(&state).await?;
        let result = match (payload.vp_token(), payload.refusal()) {
            (Some(vp_token), _) => self
                .verifier()
                .verify_all(&mut verification, vp_token)
                .await
                .map(|_| ()),
            // The wallet refused to present (it lacks the credentials, the user declined): a
            // failed verification, like an invalid presentation.
            (None, refusal) => {
                verification.ended_at = Some(Utc::now());
                verification.status = VerificationStatus::Failed;
                Err(refusal.unwrap_or_else(|| Errors::forbidden("no presentation", None)))
            }
        };

        let verification = self.repo().recv_verification().update(verification).await?;

        let mut issuance = self.repo().issuance().get_by_id(&verification.id).await?;
        issuance.build_ctx.holder_did = verification.holder;
        issuance.build_ctx.vcs = verification.vcs;
        let _issuance = self.repo().issuance().update(issuance).await?;

        let interaction = self
            .repo()
            .recv_interaction()
            .get_by_id(&verification.id)
            .await?;
        self.gatekeeper()
            .finish_interaction(&interaction, result)
            .await
    }
}
