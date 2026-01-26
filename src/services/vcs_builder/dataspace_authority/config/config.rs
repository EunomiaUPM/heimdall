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

use tracing::error;
use ymir::errors::{ErrorLogTrait, Errors};
use ymir::types::issuing::VcModel;
use ymir::types::vcs::W3cDataModelVersion;

use super::config_trait::DataSpaceAuthorityConfigTrait;
use crate::config::CoreApplicationConfig;
use crate::services::vcs_builder::min_builder_config::MinConfig;
use crate::services::vcs_builder::ConfigMinTrait;

pub struct DataSpaceAuthorityConfig {
    min_config: MinConfig,
    dataspace_id: String,
    federated_catalog_uri: String
}

impl DataSpaceAuthorityConfigTrait for DataSpaceAuthorityConfig {
    fn get_dataspace_id(&self) -> &str { &self.dataspace_id }
    fn get_catalog(&self) -> &str { &self.federated_catalog_uri }
}

impl ConfigMinTrait for DataSpaceAuthorityConfig {
    fn get_vc_model(&self) -> &VcModel { self.min_config.get_vc_model() }

    fn get_w3c_data_model(&self) -> &Option<W3cDataModelVersion> {
        self.min_config.get_w3c_data_model()
    }
}

impl From<CoreApplicationConfig> for DataSpaceAuthorityConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        let dataspace_id = value.stuff_to_issue.dataspace_id.unwrap_or_else(|| {
            let error =
                Errors::module_new("dataspace_id is not defined while being dataspace_authority");
            error!("{}", error.log());
            panic!("Cannot work as a dataspace_authority as dataspace_id is not defined")
        });

        let federated_catalog_uri =
            value.stuff_to_issue.federated_catalog_uri.unwrap_or_else(|| {
                let error = Errors::module_new(
                    "federated_catalog_uri is not defined while being dataspace_authority"
                );
                error!("{}", error.log());
                panic!(
                    "Cannot work as a dataspace_authority as federated_catalog_uri is not defined"
                )
            });

        Self {
            min_config: MinConfig {
                vc_model: value.stuff_to_issue.vc_model,
                vc_data_model: value.stuff_to_issue.w3c_data_model
            },
            dataspace_id,
            federated_catalog_uri
        }
    }
}
