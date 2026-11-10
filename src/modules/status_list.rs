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

use std::collections::HashMap;

use async_trait::async_trait;
use chrono::Utc;
use serde::Serialize;
use ymir::capabilities::DEFAULT_STATUS_LIST_TTL_MS;
use ymir::config::types::HostType;
use ymir::errors::{Errors, Outcome};
use ymir::services::{HasIssuer, HasWallet};
use ymir::types::oauth::{RoleTrait, UserInfo};
use ymir::types::vcs::doc::{
    BITSTRING_STATUS_LIST_CREDENTIAL, BitstringStatusList, BitstringStatusListEntry,
    StatusPurpose, VcDocument, VcDocumentBuilder, VcStatus,
};
use ymir::types::vcs::{Bitstring, VcIssuer, VcJwtClaimsBuilder, VcType, W3cDataModelVersion};
use ymir::utils::OneOrMany;

use crate::config::CoreConfigTrait;
use crate::data::entities::vc_status::Model;
use crate::http::routes::status_lists;
use crate::services::repo::STATUS_LIST_CAPACITY;
use crate::services::{HasConfig, HasEvents, HasRepo};

/// An issued credential with its status and, if signed, its JWT.
#[derive(Serialize)]
pub struct IssuedVcDetails {
    #[serde(flatten)]
    pub status: Model,
    pub credential: Option<String>,
}

#[derive(Clone, Copy)]
pub enum StatusChange {
    Revoke,
    Suspend,
    Reinstate,
}

impl StatusChange {
    pub fn action(&self) -> &'static str {
        match self {
            StatusChange::Revoke => "revoke",
            StatusChange::Suspend => "suspend",
            StatusChange::Reinstate => "reinstate",
        }
    }
}

#[async_trait]
pub trait StatusListModule:
    HasRepo + HasIssuer + HasWallet + HasConfig + HasEvents + Send + Sync + 'static
{
    #[tracing::instrument(level = "info", skip_all, err)]
    async fn status_list_credential(&self, purpose: &str, list_no: i32) -> Outcome<String> {
        let purpose = match purpose.parse() {
            Ok(p @ (StatusPurpose::Revocation | StatusPurpose::Suspension)) => p,
            _ => {
                return Err(Errors::missing_resource(
                    purpose,
                    "Unknown status list purpose",
                    None,
                ));
            }
        };
        if list_no < 0 || list_no > self.repo().vc_status().last_list_no().await? {
            return Err(Errors::missing_resource(
                list_no.to_string(),
                "Unknown status list",
                None,
            ));
        }

        let url = status_list_url(&*self.config(), purpose, list_no);
        let mut bits = Bitstring::new(STATUS_LIST_CAPACITY as usize);
        for idx in self.repo().vc_status().marked_indices(list_no, purpose).await? {
            bits.set(idx as usize, true)?;
        }
        let subject =
            BitstringStatusList::new(&url, purpose, bits.encode()?, Some(status_list_ttl_ms()));

        let did = self.wallet().get_did().await?;
        let now = Utc::now();
        let doc = VcDocumentBuilder::new(
            &VcType::Other(BITSTRING_STATUS_LIST_CREDENTIAL.to_string()),
            W3cDataModelVersion::default(),
        )
        .id(&url)
        .issuer(VcIssuer::new(did.id(), Some("HeimdallAuthority")))
        .credential_subject(serde_json::to_value(&subject)?)
        .valid_from(now)
        .build();
        let claims = VcJwtClaimsBuilder::new(W3cDataModelVersion::default())
            .iss(did.id())
            .jti(&url)
            .iat(now)
            .vc(doc)
            .build();

        Ok(self.issuer().sign_claims(&claims).await?.as_str().to_string())
    }

    #[tracing::instrument(level = "info", skip_all, err, fields(user = %user.id()))]
    async fn get_all_issued(&self, user: &UserInfo) -> Outcome<Vec<Model>> {
        let grants: HashMap<String, _> = self
            .repo()
            .recv_grant()
            .get_all(None, None)
            .await?
            .into_iter()
            .map(|grant| (grant.id.clone(), grant))
            .collect();
        let issued = self.repo().vc_status().get_all().await?;
        Ok(issued
            .into_iter()
            .filter(|vc| {
                grants
                    .get(&vc.issuance_id)
                    .is_some_and(|grant| user.sees_team(&grant.role, &grant.visibility))
            })
            .collect())
    }

    #[tracing::instrument(level = "info", skip_all, err, fields(user = %user.id()))]
    async fn get_issued(&self, user: &UserInfo, id: String) -> Outcome<IssuedVcDetails> {
        let grant = self.repo().recv_grant().get_by_id(&id).await?;
        user.ensure_sees_team(&grant.role, &grant.visibility, &id)?;
        let status = self.repo().vc_status().get_by_issuance(&id).await?;
        let issuance = self.repo().issuance().get_by_id(&id).await?;
        Ok(IssuedVcDetails {
            status,
            credential: issuance.credential,
        })
    }

    #[tracing::instrument(level = "info", skip_all, err, fields(user = %user.id(), action = change.action()))]
    async fn change_status(
        &self,
        user: &UserInfo,
        id: String,
        change: StatusChange,
    ) -> Outcome<Model> {
        let grant = self.repo().recv_grant().get_by_id(&id).await?;
        user.ensure_handles(&grant.role, &id)?;
        let mut status = self.repo().vc_status().get_by_issuance(&id).await?;
        if status.revoked_at.is_some() {
            return Err(Errors::forbidden("The credential is revoked", None));
        }

        match change {
            StatusChange::Revoke => status.revoked_at = Some(Utc::now()),
            StatusChange::Suspend => status.suspended_at = status.suspended_at.or(Some(Utc::now())),
            StatusChange::Reinstate => status.suspended_at = None,
        }
        let status = self.repo().vc_status().update(status).await?;
        self.events().vc_status(&grant, &status, change.action()).await;
        Ok(status)
    }
}

/// `<host><api>/status-lists/<purpose>/<list_no>`.
pub fn status_list_url(config: &dyn CoreConfigTrait, purpose: StatusPurpose, list_no: i32) -> String {
    format!(
        "{}{}{}/{}/{}",
        config.get_host(HostType::Http),
        config.get_api_version(),
        status_lists::PREFIX,
        purpose,
        list_no
    )
}

/// The revocation and suspension entries of the credential at `idx` of list `list_no`.
pub fn status_entries(config: &dyn CoreConfigTrait, list_no: i32, idx: i32) -> OneOrMany<VcStatus> {
    let entries = [StatusPurpose::Revocation, StatusPurpose::Suspension]
        .into_iter()
        .map(|purpose| {
            let url = status_list_url(config, purpose, list_no);
            BitstringStatusListEntry::new(&url, purpose, idx as u64).into()
        })
        .collect();
    OneOrMany::Many(entries)
}

pub fn issued_types(doc: &VcDocument) -> String {
    doc.r#type
        .iter()
        .filter(|t| t.as_str() != "VerifiableCredential")
        .cloned()
        .collect::<Vec<_>>()
        .join(", ")
}

fn status_list_ttl_ms() -> u64 {
    std::env::var("STATUS_LIST_TTL_MS")
        .ok()
        .and_then(|ttl| ttl.parse().ok())
        .unwrap_or(DEFAULT_STATUS_LIST_TTL_MS)
}
