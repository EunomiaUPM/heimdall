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
use serde_json::Value;
use tracing::{error, info};
use ymir::data::entities::{issuing, vc_request};
use ymir::errors::{ErrorLogTrait, Errors};
use ymir::types::vcs::vc_specs::dataspace::DataSpaceParticipant;
use ymir::types::vcs::VcType;
use ymir::utils::get_from_opt;

use super::super::VcBuilderTrait;
use crate::services::vcs_builder::dataspace_authority::config::{
    DataSpaceAuthorityConfig, DataSpaceAuthorityConfigTrait
};

pub struct DataSpaceAuthorityVcBuilder {
    config: DataSpaceAuthorityConfig
}

impl DataSpaceAuthorityVcBuilder {
    pub fn new(config: DataSpaceAuthorityConfig) -> Self { Self { config } }
}

impl VcBuilderTrait for DataSpaceAuthorityVcBuilder {
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
        let dataspace_id = self.config.get_dataspace_id().to_string();
        let fed_catalog_uri = self.config.get_catalog().to_string();

        let cred_subj = DataSpaceParticipant::new(holder_did, dataspace_id, fed_catalog_uri);

        let credential_subject = serde_json::to_value(&cred_subj)?;
        self.just_build(&model, credential_subject, &self.config)
    }

    fn gather_data(&self, _req_model: &vc_request::Model) -> anyhow::Result<String> {
        Ok("WE DONT NEED DATA".to_string())
    }
}
