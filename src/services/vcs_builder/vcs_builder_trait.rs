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

use anyhow::bail;
use chrono::{Duration, Utc};
use serde_json::Value;
use tracing::error;
use ymir::data::entities::{issuing, vc_request};
use ymir::errors::{ErrorLogTrait, Errors};
use ymir::types::issuing::VcModel;
use ymir::types::vcs::claims_v1::{VCClaimsV1, VCFromClaimsV1};
use ymir::types::vcs::claims_v2::VCClaimsV2;
use ymir::types::vcs::vc_issuer::VCIssuer;
use ymir::types::vcs::{VcType, W3cDataModelVersion};
use ymir::utils::get_from_opt;

use crate::services::vcs_builder::ConfigMinTrait;

pub trait VcBuilderTrait: Send + Sync + 'static {
    fn build_vc(&self, model: &issuing::Model) -> anyhow::Result<Value>;
    fn gather_data(&self, req_model: &vc_request::Model) -> anyhow::Result<String>;
    fn just_build(
        &self,
        model: &issuing::Model,
        credential_subject: Value,
        config: &dyn ConfigMinTrait
    ) -> anyhow::Result<Value> {
        let now = Utc::now();
        let vc_type = VcType::from_str(&model.vc_type)?;
        let issuer_did = get_from_opt(&model.issuer_did, "issuer did")?;
        match config.get_vc_model() {
            VcModel::JwtVc => {
                let w3c_data_model = config.get_w3c_data_model().as_ref().ok_or_else(|| {
                    let error = Errors::module_new("vc_jwt format is not active");
                    error!("{}", error.log());
                    error
                })?;

                let vc = match w3c_data_model {
                    W3cDataModelVersion::V1 => serde_json::to_value(VCClaimsV1 {
                        exp: None,
                        iat: None,
                        iss: None,
                        sub: None,
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
                    W3cDataModelVersion::V2 => serde_json::to_value(VCClaimsV2 {
                        exp: None,
                        iat: None,
                        iss: None,
                        sub: None,
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
            VcModel::SdJwtVc => {
                let error =
                    Errors::not_impl_new("sdj_jwt", "Cannot issue vcs  with this format right now");
                error!("{}", error.log());
                bail!(error)
            }
        }
    }
}
