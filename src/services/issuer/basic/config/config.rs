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

use crate::config::{CoreApplicationConfig, CoreConfigTrait};
use crate::services::issuer::basic::config::config_trait::BasicIssuerConfigTrait;
use crate::types::enums::data_model::W3cDataModelVersion;
use crate::types::host::{HostConfig, HostConfigTrait};
use crate::types::wallet::DidConfig;

pub struct BasicIssuerConfig {
    host: HostConfig,
    is_local: bool,
    api_path: String,
    w3c_vc_data_model: Option<W3cDataModelVersion>,
    did_config: DidConfig
}

impl From<CoreApplicationConfig> for BasicIssuerConfig {
    fn from(config: CoreApplicationConfig) -> BasicIssuerConfig {
        let api_path = config.get_api_path();
        BasicIssuerConfig {
            host: config.host,
            is_local: config.is_local,
            api_path,
            w3c_vc_data_model: config.stuff_to_issue.w3c_data_model,
            did_config: config.did_config
        }
    }
}

impl BasicIssuerConfigTrait for BasicIssuerConfig {
    fn get_host_without_protocol(&self) -> String { self.host.get_host_without_protocol() }

    fn get_host(&self) -> String { self.host.get_host() }

    fn is_local(&self) -> bool { self.is_local }

    fn get_api_path(&self) -> String { self.api_path.clone() }
    fn get_w3c_data_model(&self) -> Option<W3cDataModelVersion> { self.w3c_vc_data_model.clone() }
    fn get_did(&self) -> String { self.did_config.did.clone() }
}
