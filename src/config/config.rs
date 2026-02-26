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

use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use tracing::debug;
use ymir::config::traits::{
    ApiConfigTrait, ConnectionConfigTrait, DatabaseConfigTrait, DidConfigTrait, HostsConfigTrait,
    IssueConfigTrait, VcConfigTrait, VerifyReqConfigTrait, WalletConfigTrait,
};
use ymir::config::types::{
    ApiConfig, CommonHostsConfig, ConnectionConfig, DatabaseConfig, DidConfig, IssueConfig,
    VcConfig, VerifyReqConfig, WalletConfig,
};
use ymir::errors::{Errors, Outcome};
use ymir::utils::read;

use super::CoreConfigTrait;
use crate::config::role::{AuthorityRole, RoleConfigTrait};

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
    is_react: bool,
}

impl CoreApplicationConfig {
    pub fn load(env_file: String) -> Outcome<Self> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(env_file);
        debug!("Config file path: {}", path.display());

        let data = read(path)?;
        serde_norway::from_str(&data)
            .map_err(|e| Errors::parse("Unable to parse config file", Some(Box::new(e))))
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

impl DidConfigTrait for CoreApplicationConfig {
    fn did_config(&self) -> &DidConfig {
        &self.did_config
    }
}

impl IssueConfigTrait for CoreApplicationConfig {
    fn issue_config(&self) -> &IssueConfig {
        &self.issue_config
    }
}

impl VerifyReqConfigTrait for CoreApplicationConfig {
    fn verify_req_config(&self) -> &VerifyReqConfig {
        &self.verify_req_config
    }
}

impl VcConfigTrait for CoreApplicationConfig {
    fn vc_config(&self) -> &VcConfig {
        &self.vc_config
    }
}

impl WalletConfigTrait for CoreApplicationConfig {
    fn wallet_config(&self) -> &WalletConfig {
        self.wallet_config.as_ref().expect("Module wallet is not active")
    }
}

impl RoleConfigTrait for CoreApplicationConfig {
    fn get_role(&self) -> &AuthorityRole {
        &self.role
    }
}

impl CoreConfigTrait for CoreApplicationConfig {
    fn is_wallet_active(&self) -> bool {
        self.wallet_config.is_some()
    }

    fn is_react(&self) -> bool {
        self.is_react
    }
}
