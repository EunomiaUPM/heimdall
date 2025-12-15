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
use super::config_trait::DataSpaceAuthorityConfigTrait;
use crate::config::CoreApplicationConfig;
use crate::errors::{ErrorLogTrait, Errors};
use crate::types::enums::data_model::W3cDataModelVersion;
use crate::types::issuing::VcModel;
use tracing::error;

pub struct DataSpaceAuthorityConfig {
    vc_model: VcModel,
    vc_data_model: Option<W3cDataModelVersion>,
    dataspace_id: String,
}

impl DataSpaceAuthorityConfigTrait for DataSpaceAuthorityConfig {
    fn get_w3c_data_model(&self) -> &Option<W3cDataModelVersion> {
        &self.vc_data_model
    }
    fn get_dataspace_id(&self) -> &str {
        &self.dataspace_id
    }
    fn get_vc_model(&self) -> &VcModel {
        &self.vc_model
    }
}

impl From<CoreApplicationConfig> for DataSpaceAuthorityConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        let dataspace_id = match value.stuff_to_issue.dataspace_id {
            Some(dataspace_id) => Some(dataspace_id),
            None => {
                let error = Errors::module_new("dataspace_authority");
                error!("{}", error.log());
                None
            }
        };

        let dataspace_id = dataspace_id.expect("Module not active");

        Self {
            vc_model: value.stuff_to_issue.vc_model,
            vc_data_model: value.stuff_to_issue.w3c_data_model,
            dataspace_id,
        }
    }
}
