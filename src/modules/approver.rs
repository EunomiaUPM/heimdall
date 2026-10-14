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
use serde_json::{json, Value};
use ymir::data::entities::received::grant::Model;
use ymir::errors::{Errors, Outcome};
use ymir::types::gnap::grant_request::GrantKind;
use ymir::types::gnap::VcDecisionApproval;
use ymir::types::gnap::{GrantStatus, InteractionFinishResponse};
use ymir::types::oauth::{RoleTrait, UserInfo};

#[async_trait]
pub trait ApproverModule: HasRepo + HasGateKeeper + Send + Sync + 'static {
    async fn manage_req(
        &self,
        user: &UserInfo,
        id: String,
        payload: VcDecisionApproval,
    ) -> Outcome<InteractionFinishResponse> {
        let mut grant = self.repo().recv_grant().get_by_id(&id).await?;
        user.ensure_handles(&grant.role, &id)?;
        let interaction = self.repo().recv_interaction().get_by_id(&id).await?;

        let result: Outcome<()> = if payload.approve {
            grant.status = GrantStatus::Approved;
            Ok(())
        } else {
            grant.status = GrantStatus::Rejected;
            grant.ended_at = Some(Utc::now());
            Err(Errors::crazy(
                "Petition rejected due to internal decisions",
                None,
            ))
        };
        self.repo().recv_grant().update(grant).await?;

        self.gatekeeper()
            .finish_interaction(&interaction, result)
            .await
    }
    // =================================== GETTERS FOR FRONTEND ====================================
    async fn get_all(&self, user: &UserInfo) -> Outcome<Vec<Model>> {
        let grants = self.repo().recv_grant().get_all(None, None).await?;
        Ok(grants
            .into_iter()
            .filter(|grant| grant.kind == GrantKind::CredentialRequest)
            .filter(|grant| user.sees_team(&grant.role, &grant.visibility))
            .collect())
    }
    async fn get_by_id(&self, user: &UserInfo, id: String) -> Outcome<Model> {
        let grant = self.repo().recv_grant().get_by_id(&id).await?;
        user.ensure_sees_team(&grant.role, &grant.visibility, &id)?;
        Ok(grant)
    }

    async fn get_by_id_with_details(&self, user: &UserInfo, id: String) -> Outcome<Value> {
        let grant = self.repo().recv_grant().get_by_id(&id).await?;
        user.ensure_sees_team(&grant.role, &grant.visibility, &id)?;
        let issuance = self.repo().issuance().get_by_id(&id).await?;
        let interaction = self.repo().recv_interaction().get_by_id(&id).await.ok();
        let verification = self.repo().recv_verification().get_by_id(&id).await.ok();
        Ok(json!({
            "grant": grant,
            "issuance": issuance,
            "interaction": interaction,
            "verification": verification,
        }))
    }
}
