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

use ymir::config::traits::ApiConfigTrait;
use ymir::services::issuer::basic::config::{BasicIssuerConfig, BasicIssuerConfigBuilder};
use ymir::services::verifier::basic::config::{BasicVerifierConfig, BasicVerifierConfigBuilder};
use ymir::services::wallet::walt_id::config::{WaltIdConfig, WaltIdConfigBuilder};

use crate::config::CoreApplicationConfig;

impl From<CoreApplicationConfig> for WaltIdConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        WaltIdConfigBuilder::new()
            .hosts(value.hosts.clone())
            .ssi_wallet_config(value.wallet_config.unwrap())
            .did_config(value.did_config)
            .build()
    }
}

impl From<CoreApplicationConfig> for BasicVerifierConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        let api_path = value.get_api_version();
        BasicVerifierConfigBuilder::new()
            .hosts(value.hosts.clone())
            .is_local(value.is_local)
            .requested_vcs(value.verify_req_config.vcs_requested)
            .api_path(api_path)
            .build()
    }
}

impl From<CoreApplicationConfig> for BasicIssuerConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        let api_path = value.get_api_version();
        BasicIssuerConfigBuilder::new()
            .hosts(value.hosts.clone())
            .is_local(value.is_local)
            .api_path(api_path)
            .did_config(value.did_config.clone())
            .build()
    }
}
