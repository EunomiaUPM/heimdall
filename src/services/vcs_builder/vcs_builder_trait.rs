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
use ymir::data::entities::{issuing, vc_request};
use ymir::errors::{BadFormat, Errors, Outcome};
use ymir::types::jwt::VcJwtClaimsBuilder;
use ymir::types::vcs::doc::VcDocumentBuilder;
use ymir::types::vcs::{VcIssuer, VcModel, VcType};
use ymir::utils::get_from_opt;

use crate::config::traits::RoleConfigTrait;
use crate::services::vcs_builder::BuilderConfigDefaultTrait;

pub trait VcBuilderTrait: RoleConfigTrait + Send + Sync + 'static {
    fn build_vc(&self, model: &issuing::Model) -> Outcome<Value>;
    fn gather_data(&self, req_model: &vc_request::Model) -> Outcome<String>;
    fn validate(&self, vc_type: &str) -> Outcome<VcType>;
    fn just_build(
        &self,
        model: &issuing::Model,
        credential_subject: Value,
        config: &dyn BuilderConfigDefaultTrait,
    ) -> Outcome<Value> {
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
        let vc_type = VcType::from_str(&model.vc_type)?;
        let issuer_did = get_from_opt(model.issuer_did.as_ref(), "issuer did")?;
        match config.get_vc_model() {
            VcModel::JwtVc => {
                let w3c_data_model = config
                    .get_w3c_data_model()
                    .ok_or_else(|| Errors::not_active("vc_jwt format is not active", None))?;

                let doc = VcDocumentBuilder::new(&vc_type, &w3c_data_model)
                    .id(model.credential_id.clone())
                    .issuer(VcIssuer::new(&issuer_did, Some("HeimdallAuthority")))
                    .credential_subject(credential_subject)
                    .valid_from(now)
                    .valid_until(now + Duration::days(365))
                    .build();

                let vc = VcJwtClaimsBuilder::new(w3c_data_model)
                    .iss(issuer_did.clone())
                    .sub(subject_id)
                    .jti(model.credential_id.clone())
                    .iat(now)
                    .exp(now + Duration::days(365))
                    .vc(doc)
                    .build();

                Ok(serde_json::to_value(&vc)?)
            }
            VcModel::SdJwtVc => Err(Errors::not_impl(
                "Cannot issue vcs with the format 'sd_jwt' right now",
                None,
            )),
        }
    }
}
