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

use ymir::config::traits::{ApiConfigTrait, ConnectionConfigTrait, HostsConfigTrait};
use ymir::services::issuer::basic::config::{BasicIssuerConfig, BasicIssuerConfigBuilder};
use ymir::services::verifier::basic::config::{BasicVerifierConfig, BasicVerifierConfigBuilder};
use ymir::services::wallet::walt_id::config::{WaltIdConfig, WaltIdConfigBuilder};

use crate::config::{CoreApplicationConfig, CoreConfigTrait};

impl From<CoreApplicationConfig> for WaltIdConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        WaltIdConfigBuilder::new()
            .hosts(value.hosts().clone())
            .ssi_wallet_config(value.get_wallet_config().clone())
            .did_config(value.get_did_config().clone())
            .build()
    }
}

impl From<CoreApplicationConfig> for BasicVerifierConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        BasicVerifierConfigBuilder::new()
            .hosts(value.hosts().clone())
            .is_local(value.is_local())
            .requested_vcs(value.get_verify_req_config().vcs_requested.clone())
            .api_path(value.get_api_version())
            .build()
    }
}

impl From<CoreApplicationConfig> for BasicIssuerConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        BasicIssuerConfigBuilder::new()
            .hosts(value.hosts().clone())
            .is_local(value.is_local())
            .api_path(value.get_api_version())
            .did_config(value.get_did_config().clone())
            .build()
    }
}
