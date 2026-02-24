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

use chrono::{Duration, Utc};
use serde_json::Value;
use ymir::data::entities::{issuing, vc_request};
use ymir::errors::{Errors, Outcome};
use ymir::types::errors::BadFormat;
use ymir::types::vcs::claims_v1::{VCClaimsV1, VCFromClaimsV1};
use ymir::types::vcs::claims_v2::VCClaimsV2;
use ymir::types::vcs::vc_issuer::VCIssuer;
use ymir::types::vcs::{VcModel, VcType, W3cDataModelVersion};
use ymir::utils::{get_from_opt, parse_to_value};

use crate::config::role::RoleConfigTrait;
use crate::services::vcs_builder::BuilderConfigDefaultTrait;

pub trait VcBuilderTrait: RoleConfigTrait + Send + Sync + 'static {
    fn build_vc(&self, model: &issuing::Model) -> Outcome<Value>;
    fn gather_data(&self, req_model: &vc_request::Model) -> Outcome<String>;
    fn just_build(
        &self,
        model: &issuing::Model,
        credential_subject: Value,
        config: &dyn BuilderConfigDefaultTrait
    ) -> Outcome<Value> {
        let subject_id =
            credential_subject.get("id").and_then(|v| v.as_str()).ok_or_else(|| {
                Errors::format(
                    BadFormat::Received,
                    "Unable to retrieve credential subject id",
                    None
                )
            })?;

        let now = Utc::now();
        let vc_type = VcType::from_str(&model.vc_type)?;
        let issuer_did = get_from_opt(model.issuer_did.as_ref(), "issuer did")?;
        match config.get_vc_model() {
            VcModel::JwtVc => {
                let w3c_data_model = config
                    .get_w3c_data_model()
                    .ok_or_else(|| Errors::not_active("vc_jwt format is not active", None))?;

                let vc = match w3c_data_model {
                    W3cDataModelVersion::V1 => parse_to_value(&VCClaimsV1 {
                        exp: None,
                        jti: Some(model.credential_id.clone()),
                        iat: None,
                        iss: Some(issuer_did.clone()),
                        sub: Some(subject_id.to_string()),
                        vc: VCFromClaimsV1 {
                            context: vec!["https://www.w3.org/ns/credentials/v1".to_string()],
                            r#type: vec!["VerifiableCredential".to_string(), vc_type.name()],
                            id: model.credential_id.clone(),
                            credential_subject,
                            issuer: VCIssuer {
                                id: issuer_did,
                                name: Some("RainbowAuthority".to_string())
                            },
                            valid_from: Some(now),
                            valid_until: Some(now + Duration::days(365))
                        }
                    })?,
                    W3cDataModelVersion::V2 => parse_to_value(&VCClaimsV2 {
                        exp: None,
                        iat: None,
                        jti: Some(model.credential_id.clone()),
                        iss: Some(issuer_did.clone()),
                        sub: Some(subject_id.to_string()),
                        context: vec!["https://www.w3.org/ns/credentials/v2".to_string()],
                        r#type: vec!["VerifiableCredential".to_string(), vc_type.name()],
                        id: model.credential_id.clone(),
                        credential_subject,
                        issuer: VCIssuer {
                            id: issuer_did,
                            name: Some("RainbowAuthority".to_string())
                        },
                        valid_from: Some(now),
                        valid_until: Some(now + Duration::days(365))
                    })?
                };
                Ok(vc)
            }
            VcModel::SdJwtVc => Err(Errors::not_impl(
                "Cannot issue vcs with the format 'sd_jwt' right now",
                None
            ))
        }
    }
}
