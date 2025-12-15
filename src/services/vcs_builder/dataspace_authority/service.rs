/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use super::super::VcBuilderTrait;
use crate::data::entities::{issuing, request};
use crate::errors::{ErrorLogTrait, Errors};
use crate::services::vcs_builder::dataspace_authority::config::{
    DataSpaceAuthorityConfig, DataSpaceAuthorityConfigTrait,
};
use crate::types::enums::data_model::VcDataModelVersion;
use crate::types::enums::vc_type::VcType;
use crate::types::vcs::dataspace::DataSpaceParticipant;
use crate::types::vcs::{VCClaimsV1, VCClaimsV2, VCFromClaimsV1, VCIssuer};
use crate::utils::get_from_opt;
use anyhow::bail;
use chrono::{Duration, Utc};
use serde_json::Value;
use std::str::FromStr;
use tracing::{error, info};

pub struct DataSpaceAuthorityBuilder {
    config: DataSpaceAuthorityConfig,
}

impl DataSpaceAuthorityBuilder {
    pub fn new(config: DataSpaceAuthorityConfig) -> Self {
        Self { config }
    }
}

impl VcBuilderTrait for DataSpaceAuthorityBuilder {
    fn build_vc(&self, model: &issuing::Model) -> anyhow::Result<Value> {
        let vc_type = VcType::from_str(&model.vc_type)?;

        match vc_type {
            VcType::DataspaceParticipant => {}
            _ => {
                let error = Errors::unauthorized_new(&format!(
                    "Cannot issue vc_type: {}",
                    vc_type.to_string()
                ));
                error!("{}", error.log());
                bail!(error)
            }
        }

        info!("Building {} credential", vc_type.to_string());

        let holder_did = get_from_opt(&model.holder_did, "holder did")?;
        let issuer_did = get_from_opt(&model.issuer_did, "issuer did")?;
        let dataspace_id = self.config.get_dataspace_id().to_string();

        let cred_subj = DataSpaceParticipant::new(holder_did, dataspace_id);

        let credential_subject = serde_json::to_value(&cred_subj)?;
        let now = Utc::now();
        let vc_type = VcType::from_str(&model.vc_type)?;
        let vc = match self.config.get_data_model() {
            VcDataModelVersion::V1 => serde_json::to_value(VCClaimsV1 {
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
                        name: "RainbowAuthority".to_string(),
                    },
                    valid_from: Some(now),
                    valid_until: Some(now + Duration::days(365)),
                },
            })?,
            VcDataModelVersion::V2 => serde_json::to_value(VCClaimsV2 {
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
                    name: "RainbowAuthority".to_string(),
                },
                valid_from: Some(now),
                valid_until: Some(now + Duration::days(365)),
            })?,
        };

        Ok(vc)
    }

    fn gather_data(&self, _req_model: &request::Model) -> anyhow::Result<String> {
        Ok("".to_string())
    }
}
