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

use chrono::{Duration, Utc};
use serde_json::Value;
use ymir::data::entities::shared::issuance;
use ymir::errors::{BadFormat, Errors, Outcome};
use ymir::types::jwt::{VCJwtClaims, VcJwtClaimsBuilder};
use ymir::types::vcs::doc::VcDocumentBuilder;
use ymir::types::vcs::{VcIssuer, VcTypeConfig, W3cDataModelVersion};

use crate::config::traits::RoleConfigTrait;

pub trait VcBuilderTrait: RoleConfigTrait + Send + Sync + 'static {
    fn build_vc(&self, model: &issuance::Model, vc_config: VcTypeConfig) -> Outcome<VCJwtClaims>;
    fn just_build(
        &self,
        issuance: &issuance::Model,
        credential_subject: Value,
        vc_config: VcTypeConfig,
    ) -> Outcome<VCJwtClaims> {
        let subject_id = credential_subject
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                Errors::format(
                    BadFormat::Received,
                    "Unable to retrieve credential subject id",
                    None,
                )
            })?
            .to_string();

        let now = Utc::now();

        let doc = VcDocumentBuilder::new(vc_config.vc_type(), W3cDataModelVersion::default())
            .id(issuance.credential_id.clone())
            .issuer(VcIssuer::new(
                issuance.issuer_did.clone(),
                Some("HeimdallAuthority"),
            ))
            .credential_subject(credential_subject)
            .valid_from(now)
            .valid_until(now + Duration::days(365))
            .build();

        let vc = VcJwtClaimsBuilder::new(W3cDataModelVersion::default())
            .iss(issuance.issuer_did.clone().clone())
            .sub(subject_id)
            .jti(issuance.credential_id.clone())
            .iat(now)
            .exp(now + Duration::days(365))
            .vc(doc)
            .build();

        Ok(vc)
    }
}
