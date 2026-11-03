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

//! Event payloads: what subscribers may see of each record, never its tokens or codes.

use chrono::{DateTime, Utc};
use serde::Serialize;
use ymir::data::entities::received::{grant, interaction, verification};
use ymir::data::entities::shared::issuance;
use ymir::types::gnap::grant_request::GrantKind;
use ymir::types::gnap::GrantStatus;
use ymir::types::vcs::{VcType, VcTypeConfig};
use ymir::types::verification::VerificationStatus;

#[derive(Serialize)]
pub struct GrantEvent {
    pub grant_id: String,
    pub kind: GrantKind,
    pub participant_id: Option<String>,
    pub participant_nick: String,
    pub status: GrantStatus,
    pub vc_type_config: Option<Vec<VcTypeConfig>>,
    pub created_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

impl From<&grant::Model> for GrantEvent {
    fn from(grant: &grant::Model) -> Self {
        Self {
            grant_id: grant.id.clone(),
            kind: grant.kind.clone(),
            participant_id: grant.participant_id.clone(),
            participant_nick: grant.participant_nick.clone(),
            status: grant.status.clone(),
            vc_type_config: grant.vc_type_config.clone(),
            created_at: grant.created_at,
            ended_at: grant.ended_at,
        }
    }
}

#[derive(Serialize)]
pub struct IssuanceEvent {
    pub grant_id: String,
    pub subject_name: String,
    pub vc_type_config: Vec<VcTypeConfig>,
    pub issuer_did: String,
    pub credential_id: String,
    pub holder_did: Option<String>,
    /// Whether the credential has been signed and handed out.
    pub issued: bool,
}

impl From<&issuance::Model> for IssuanceEvent {
    fn from(issuance: &issuance::Model) -> Self {
        Self {
            grant_id: issuance.id.clone(),
            subject_name: issuance.subject_name.clone(),
            vc_type_config: issuance.vc_type_config.clone(),
            issuer_did: issuance.issuer_did.clone(),
            credential_id: issuance.credential_id.clone(),
            holder_did: issuance.build_ctx.holder_did.clone(),
            issued: issuance.credential.is_some(),
        }
    }
}

#[derive(Serialize)]
pub struct InteractionEvent {
    pub grant_id: String,
    pub method: String,
    pub callback_uri: String,
}

impl From<&interaction::Model> for InteractionEvent {
    fn from(interaction: &interaction::Model) -> Self {
        Self {
            grant_id: interaction.grant_id.clone(),
            method: interaction.method.to_string(),
            callback_uri: interaction.callback_uri.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct VerificationEvent {
    pub grant_id: String,
    pub holder: Option<String>,
    pub vc_types: Vec<VcType>,
    pub status: VerificationStatus,
}

impl From<&verification::Model> for VerificationEvent {
    fn from(verification: &verification::Model) -> Self {
        Self {
            grant_id: verification.grant_id.clone(),
            holder: verification.holder.clone(),
            vc_types: verification.vc_type.clone(),
            status: verification.status.clone(),
        }
    }
}
