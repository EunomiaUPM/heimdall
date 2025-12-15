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
use crate::types::enums::data_model::VcDataModelVersion;
use tracing::error;

pub struct DataSpaceAuthorityConfig {
    vc_data_model: VcDataModelVersion,
    dataspace_id: String,
}

impl DataSpaceAuthorityConfigTrait for DataSpaceAuthorityConfig {
    fn get_data_model(&self) -> &VcDataModelVersion {
        &self.vc_data_model
    }
    fn get_dataspace_id(&self) -> &str {
        &self.dataspace_id
    }
}

impl From<CoreApplicationConfig> for DataSpaceAuthorityConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        let dataspace_id = match value.dataspace_id {
            Some(dataspace_id) => Some(dataspace_id),
            None => {
                let error = Errors::module_new("dataspace_authority");
                error!("{}", error.log());
                None
            }
        };

        let dataspace_id = dataspace_id.expect("Module not active");

        Self {
            vc_data_model: value.vc_data_model,
            dataspace_id,
        }
    }
}
