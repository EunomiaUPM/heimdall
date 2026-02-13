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
use tracing::{debug, error};
use ymir::config::traits::{
    ApiConfigTrait, ConnectionConfigTrait, DatabaseConfigTrait, HostsConfigTrait,
};
use ymir::config::types::{
    ApiConfig, CommonHostsConfig, ConnectionConfig, DatabaseConfig, DidConfig, IssueConfig,
    VcConfig, VerifyReqConfig, WalletConfig,
};
use ymir::errors::{ErrorLogTrait, Errors};

use super::CoreConfigTrait;
use crate::types::role::AuthorityRole;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CoreApplicationConfig {
    hosts_config: CommonHostsConfig,
    connection_config: ConnectionConfig,
    api_config: ApiConfig,
    db_config: DatabaseConfig,
    wallet_config: Option<WalletConfig>,
    did_config: DidConfig,
    issue_config: IssueConfig,
    vc_config: VcConfig,
    verify_req_config: VerifyReqConfig,
    role: AuthorityRole,
}

impl CoreApplicationConfig {
    pub fn load(env_file: String) -> Self {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(env_file);
        debug!("Config file path: {}", path.display());

        let data = fs::read_to_string(&path).expect("Unable to read config file");
        serde_norway::from_str(&data).expect("Unable to parse config file")
    }
}

impl DatabaseConfigTrait for CoreApplicationConfig {
    fn db(&self) -> &DatabaseConfig {
        &self.db_config
    }
}

impl ApiConfigTrait for CoreApplicationConfig {
    fn api(&self) -> &ApiConfig {
        &self.api_config
    }
}

impl ConnectionConfigTrait for CoreApplicationConfig {
    fn connection(&self) -> &ConnectionConfig {
        &self.connection_config
    }
}

impl HostsConfigTrait for CoreApplicationConfig {
    fn hosts(&self) -> &CommonHostsConfig {
        &self.hosts_config
    }
}

impl CoreConfigTrait for CoreApplicationConfig {
    fn get_role(&self) -> AuthorityRole {
        self.role.clone()
    }

    fn is_wallet_active(&self) -> bool {
        self.wallet_config.is_some()
    }

    fn get_wallet_config(&self) -> &WalletConfig {
        let wallet = match self.wallet_config.as_ref() {
            Some(data) => Some(data),
            None => {
                let error = Errors::module_new("wallet");
                error!("{}", error.log());
                None
            }
        };
        wallet.expect("Module wallet is no active")
    }

    fn get_did_config(&self) -> &DidConfig {
        &self.did_config
    }

    fn get_issue_config(&self) -> &IssueConfig {
        &self.issue_config
    }

    fn get_verify_req_config(&self) -> &VerifyReqConfig {
        &self.verify_req_config
    }
    fn get_vc_config(&self) -> &VcConfig {
        &self.vc_config
    }
}
