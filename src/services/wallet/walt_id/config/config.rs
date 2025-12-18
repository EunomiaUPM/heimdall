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

use super::WaltIdConfigTrait;
use crate::config::CoreApplicationConfig;
use crate::types::host::{HostConfig, HostConfigTrait};
use crate::types::wallet::WalletConfig;

pub struct WaltIdConfig {
    host: HostConfig,
    ssi_wallet_config: WalletConfig
}

impl From<CoreApplicationConfig> for WaltIdConfig {
    fn from(config: CoreApplicationConfig) -> Self {
        WaltIdConfig {
            host: config.host,
            ssi_wallet_config: config.wallet_config.clone().expect("Module not active")
        }
    }
}

impl WaltIdConfigTrait for WaltIdConfig {
    fn get_raw_wallet_config(&self) -> WalletConfig { self.ssi_wallet_config.clone() }
    fn get_wallet_host(&self) -> String { self.ssi_wallet_config.api.get_host() }
    fn get_host(&self) -> String { self.host.get_host() }
}
