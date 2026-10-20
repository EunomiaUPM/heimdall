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

use crate::config::CoreConfig;
use ymir::config::traits::{
    ApiConfigTrait, DidConfigTrait, HostsConfigTrait, VerifyReqConfigTrait, WalletConfigTrait,
};
use ymir::services::issuer::oid4vci_1_0::IssuerConfig;
use ymir::services::verifier::oid4vp_1_0;
use ymir::services::verifier::oid4vp_draft20;
use ymir::services::wallet::fafnir::FafnirConfig;
use ymir::services::wallet::walt_id::WaltIdConfig;

impl From<&CoreConfig> for WaltIdConfig {
    fn from(value: &CoreConfig) -> Self {
        WaltIdConfig::new(
            value.hosts().clone(),
            value.wallet_config().clone(),
            value.did_config().clone(),
        )
    }
}

impl From<&CoreConfig> for FafnirConfig {
    fn from(value: &CoreConfig) -> Self {
        FafnirConfig::new(
            value.hosts().clone(),
            value.wallet_config().clone(),
            value.did_config().clone(),
        )
    }
}

impl From<&CoreConfig> for oid4vp_draft20::VerifierConfig {
    fn from(value: &CoreConfig) -> Self {
        oid4vp_draft20::VerifierConfig::new(
            value.hosts().clone(),
            value.get_api_version(),
            value.get_requested_vcs().to_vec(),
        )
    }
}

impl From<&CoreConfig> for oid4vp_1_0::VerifierConfig {
    fn from(value: &CoreConfig) -> Self {
        oid4vp_1_0::VerifierConfig::new(
            value.hosts().clone(),
            value.get_api_version(),
            value.get_requested_vcs().to_vec(),
            value.get_request_passing().clone(),
        )
    }
}

impl From<&CoreConfig> for IssuerConfig {
    fn from(value: &CoreConfig) -> Self {
        IssuerConfig::new(value.hosts().clone(), value.get_api_version())
    }
}
