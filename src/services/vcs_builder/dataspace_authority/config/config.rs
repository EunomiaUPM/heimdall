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
use ymir::types::issuing::{VcConfig, VcModel};
use ymir::types::vcs::W3cDataModelVersion;

use super::config_trait::DataSpaceAuthorityConfigTrait;
use crate::config::{CoreApplicationConfig, CoreConfigTrait};
use crate::services::vcs_builder::BuilderConfigDefaultTrait;

pub struct DataSpaceAuthorityConfig {
    vc_config: VcConfig,
    dataspace_id: String,
    federated_catalog_uri: String
}

impl BuilderConfigDefaultTrait for DataSpaceAuthorityConfig {
    fn get_vc_model(&self) -> &VcModel { self.vc_config.get_vc_model() }

    fn get_w3c_data_model(&self) -> &Option<W3cDataModelVersion> {
        self.vc_config.get_w3c_data_model()
    }
}

impl DataSpaceAuthorityConfigTrait for DataSpaceAuthorityConfig {
    fn get_dataspace_id(&self) -> &str { &self.dataspace_id }
    fn get_catalog(&self) -> &str { &self.federated_catalog_uri }
}

impl From<CoreApplicationConfig> for DataSpaceAuthorityConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        let dataspace_id = value.get_issue_config().dataspace_id.clone().unwrap_or_else(|| {
            let error =
                Errors::module_new("dataspace_id is not defined while being dataspace_authority");
            error!("{}", error.log());
            panic!("Cannot work as a dataspace_authority as dataspace_id is not defined")
        });

        let federated_catalog_uri =
            value.get_issue_config().federated_catalog_uri.clone().unwrap_or_else(|| {
                let error = Errors::module_new(
                    "federated_catalog_uri is not defined while being dataspace_authority"
                );
                error!("{}", error.log());
                panic!(
                    "Cannot work as a dataspace_authority as federated_catalog_uri is not defined"
                )
            });

        Self {
            vc_config: VcConfig {
                vc_model: value.get_issue_config().vc_config.get_vc_model().clone(),
                w3c_data_model: value.get_issue_config().vc_config.get_w3c_data_model().clone()
            },
            dataspace_id,
            federated_catalog_uri
        }
    }
}
