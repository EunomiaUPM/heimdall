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

use std::path::PathBuf;
use std::{env, fs};

use serde::{Deserialize, Serialize};
use tracing::debug;
use ymir::config::traits::{ApiConfigTrait, DatabaseConfigTrait};
use ymir::config::types::{ApiConfig, CommonHostsConfig, DatabaseConfig};
use ymir::types::dids::did_config::DidConfig;
use ymir::types::issuing::StuffToIssue;
use ymir::types::verifying::RequirementsToVerify;
use ymir::types::wallet::WalletConfig;

use super::CoreConfigTrait;
use crate::types::role::AuthorityRole;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CoreApplicationConfig {
    pub hosts: CommonHostsConfig,
    pub is_local: bool,
    pub is_tls: bool,
    pub db_config: DatabaseConfig,
    pub wallet_config: Option<WalletConfig>,
    pub did_config: DidConfig,
    pub role: AuthorityRole,
    pub api: ApiConfig,
    pub stuff_to_issue: StuffToIssue,
    pub requirements_to_verify: RequirementsToVerify,
}

impl CoreApplicationConfig {
    pub fn load(env_file: String) -> Self {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(env_file);
        debug!("Config file path: {}", path.display());

        let data = fs::read_to_string(&path).expect("Unable to read config file");
        serde_norway::from_str(&data).expect("Unable to parse config file")
    }
}

impl CoreApplicationConfig {
    pub fn get_did(&self) -> String {
        self.did_config.did.clone()
    }
}

impl DatabaseConfigTrait for CoreApplicationConfig {
    fn db(&self) -> &DatabaseConfig {
        &self.db_config
    }
}

impl ApiConfigTrait for CoreApplicationConfig {
    fn api(&self) -> &ApiConfig {
        &self.api
    }
}

impl CoreConfigTrait for CoreApplicationConfig {
    fn hosts(&self) -> &CommonHostsConfig {
        &self.hosts
    }

    fn is_local(&self) -> bool {
        self.is_local
    }
    fn is_tls(&self) -> bool {
        self.is_tls
    }

    fn get_role(&self) -> AuthorityRole {
        self.role.clone()
    }

    fn is_wallet_active(&self) -> bool {
        self.wallet_config.is_some()
    }
}
