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

use crate::config::CoreApplicationConfig;
use ymir::config::traits::{
    ApiConfigTrait, DidConfigTrait, HostsConfigTrait, VcConfigTrait, VerifyReqConfigTrait,
    WalletConfigTrait,
};
use ymir::services::issuer::basic::BasicIssuerConfig;
use ymir::services::verifier::basic::BasicVerifierConfig;
use ymir::services::wallet::fafnir::FafnirConfig;
use ymir::services::wallet::walt_id::WaltIdConfig;

impl From<&CoreApplicationConfig> for WaltIdConfig {
    fn from(value: &CoreApplicationConfig) -> Self {
        WaltIdConfig::new(
            value.hosts().clone(),
            value.wallet_config().clone(),
            value.did_config().clone(),
        )
    }
}

impl From<&CoreApplicationConfig> for FafnirConfig {
    fn from(value: &CoreApplicationConfig) -> Self {
        FafnirConfig::new(
            value.hosts().clone(),
            value.wallet_config().clone(),
            value.did_config().clone(),
        )
    }
}

impl From<&CoreApplicationConfig> for BasicVerifierConfig {
    fn from(value: &CoreApplicationConfig) -> Self {
        BasicVerifierConfig::new(
            value.hosts().clone(),
            value.get_api_version(),
            value.get_requested_vcs().to_vec(),
            value.vc_config().clone(),
        )
    }
}

impl From<&CoreApplicationConfig> for BasicIssuerConfig {
    fn from(value: &CoreApplicationConfig) -> Self {
        BasicIssuerConfig::new(value.hosts().clone(), value.get_api_version())
    }
}
