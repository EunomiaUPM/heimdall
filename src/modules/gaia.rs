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

//! Gaia-X compliance by plain POST (Loire's compliance API, Danube extensions): a presentation
//! in, a compliance credential out, recorded like any other request.

use async_trait::async_trait;
use chrono::Utc;
use tracing::warn;
use uuid::Uuid;
use ymir::data::entities::received::{grant, verification};
use ymir::data::entities::shared::issuance;
use ymir::errors::{Errors, Outcome};
use ymir::services::verifier::validators::{
    check_bindings, validate_vp_holder, verify_vc, PresentedVc,
};
use ymir::services::{HasIssuer, HasWallet};
use ymir::types::dids::Did;
use ymir::types::gnap::grant_request::GrantKind;
use ymir::types::gnap::GrantStatus;
use ymir::types::participants::Visibility;
use ymir::types::jwt::{Jwt, JwtHeaderTrait};
use ymir::types::oauth::RolePath;
use ymir::types::vcs::{BuildCtx, VcType, VcTypeConfig};
use ymir::types::verification::VerificationStatus;
use ymir::types::vps::VpPayload;
use ymir::capabilities::Verifier;

use crate::data::entities::vc_status;
use crate::modules::{issued_types, status_entries};
use crate::services::{HasConfig, HasEvents, HasRepo, HasVcBuilder};

#[async_trait]
pub trait GaiaModule:
    HasIssuer + HasRepo + HasVcBuilder + HasWallet + HasEvents + HasConfig + Send + Sync + 'static
{
    /// Issues `vc_type` (a compliance credential this role issues) over the presentation
    /// `vp_token`; `vcid` names it.
    #[tracing::instrument(level = "info", skip_all, err)]
    async fn gaia_compliance(
        &self,
        vc_type: VcTypeConfig,
        vcid: Option<String>,
        vp_token: &str,
    ) -> Outcome<String> {
        let issued = self.vc_builder().get_role().available_credentials();
        if !matches!(vc_type.vc_type(), VcType::GxLabel | VcType::ParticipantCompliance)
            || !issued.contains(vc_type.vc_type())
        {
            return Err(Errors::forbidden(format!("{vc_type} is not issued here"), None));
        }
        let vp = Jwt::<VpPayload>::parse(vp_token)?;
        let claimed = vp.kid()?.did().id().to_string();
        let requested = self.config().get_requested_vcs().to_vec();
        let own = self.wallet().get_identity().read().await.did().id().to_string();

        let mut grant = self
            .repo()
            .recv_grant()
            .create(grant::Plan {
                id: Uuid::new_v4().to_string(),
                role: RolePath::root(),
                visibility: Visibility::Public,
                participant_nick: claimed.clone(),
                vc_type_config: Some(vec![vc_type.clone()]),
                kind: GrantKind::CredentialRequest,
            })
            .await?;
        let mut issuance = self
            .repo()
            .issuance()
            .create(issuance::Plan {
                role: grant.role.clone(),
                id: grant.id.clone(),
                subject_name: claimed.clone(),
                vc_type_config: vec![vc_type.clone()],
                aud: own.clone(),
                issuer_did: own.clone(),
                build_ctx: BuildCtx::base(claimed.clone(), None),
            })
            .await?;
        let mut ver = self
            .repo()
            .recv_verification()
            .create(verification::Plan {
                grant_id: grant.id.clone(),
                audience: own,
                vc_type: requested.clone(),
            })
            .await?;
        self.events().grant(&grant, "create").await;
        self.events().issuance(&grant, &issuance, "create").await;
        self.events().verification(&grant, &ver, "create").await;

        let result = verify_standalone(vp, &requested).await;
        ver.vpt = Some(vp_token.to_string());
        ver.ended_at = Some(Utc::now());
        match &result {
            Ok((holder, vcs)) => {
                ver.status = VerificationStatus::Verified;
                ver.holder = Some(holder.id().to_string());
                ver.vcs = vcs.clone();
            }
            Err(e) => {
                warn!("Gaia-X presentation rejected: {}", e.reason());
                ver.status = VerificationStatus::Failed;
            }
        }
        let ver = self.repo().recv_verification().update(ver).await?;
        self.events().verification(&grant, &ver, "edit").await;

        let built = match result {
            Ok((holder, vcs)) => {
                issuance.build_ctx.holder_did = Some(holder.id().to_string());
                issuance.build_ctx.vcs = vcs;
                if let Some(vcid) = vcid {
                    issuance.credential_id = vcid;
                }
                self.sign(&mut issuance, vc_type).await
            }
            Err(e) => Err(e),
        };

        grant.status = GrantStatus::Finalized;
        grant.ended_at = Some(Utc::now());
        let grant = self.repo().recv_grant().update(grant).await?;
        self.events().grant(&grant, "edit").await;
        let vc_jwt = built?;
        let issuance = self.repo().issuance().get_by_id(&grant.id).await?;
        self.events().issuance(&grant, &issuance, "edit").await;
        Ok(vc_jwt)
    }

    /// Builds the credential of `issuance`, with its status entry, signs and stores it.
    async fn sign(&self, issuance: &mut issuance::Model, vc_type: VcTypeConfig) -> Outcome<String> {
        let holder = issuance.build_ctx.holder_did.clone().unwrap_or_default();
        let mut claims = self.vc_builder().build_vc(issuance, vc_type)?;
        let status = self
            .repo()
            .vc_status()
            .allocate(vc_status::Plan {
                credential_id: issuance.credential_id.clone(),
                issuance_id: issuance.id.clone(),
                vc_type: issued_types(claims.vc_doc()),
                holder_did: holder,
                subject_name: issuance.subject_name.clone(),
            })
            .await?;
        claims.vc_doc_mut().credential_status =
            Some(status_entries(&*self.config(), status.list_no, status.idx));
        let vc_jwt = self.issuer().sign_claims(&claims).await?.as_str().to_string();
        issuance.credential = Some(vc_jwt.clone());
        self.repo().issuance().update(issuance.clone()).await?;
        Ok(vc_jwt)
    }
}

/// A presentation on its own (no `aud` nor `nonce`): signed by its holder, not expired, with an
/// `id`, its credentials valid and tied to the holder, and one of each `requested` type.
async fn verify_standalone(
    vp: Jwt<VpPayload>,
    requested: &[VcType],
) -> Outcome<(Did, Vec<String>)> {
    let vp = Verifier::verify_enveloped(vp, None).await?;
    validate_vp_holder(&vp)?;
    if vp.claims().exp().is_some_and(|exp| exp < Utc::now().timestamp()) {
        return Err(Errors::forbidden("The presentation has expired", None));
    }
    if vp.claims().vp_doc().id.is_empty() {
        return Err(Errors::forbidden("The presentation has no id", None));
    }
    let holder = vp.kid()?.did().clone();

    let jwts = vp.claims().vp_doc().credential_jwts()?;
    let mut presented: Vec<PresentedVc> = Vec::with_capacity(jwts.len());
    for jwt in &jwts {
        presented.push(verify_vc(jwt, &holder).await?);
    }
    check_bindings(&presented)?;
    for wanted in requested {
        let found = presented
            .iter()
            .any(|p| p.types().iter().any(|t| *t == wanted.to_string()));
        if !found {
            return Err(Errors::forbidden(format!("Missing required credential: {wanted}"), None));
        }
    }
    Ok((holder, jwts))
}
