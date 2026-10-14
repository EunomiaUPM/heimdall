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

use std::env;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tracing::debug;
use ymir::config::traits::{
    ApiConfigTrait, ConnectionConfigTrait, DatabaseConfigTrait, DidConfigTrait, HostsConfigTrait,
    VerifyReqConfigTrait, WalletConfigTrait,
};
use ymir::config::types::{
    ApiConfig, CommonHostsConfig, ConnectionConfig, DatabaseConfig, DidConfig, VerifyReqConfig,
    WalletConfig,
};
use ymir::errors::{Errors, Outcome};
use ymir::utils::read;

use super::CoreConfigTrait;
use crate::config::traits::{IssueConfigTrait, OauthConfigTrait, RoleConfigTrait};
use crate::config::types::{AuthorityRole, IssueConfig, OauthConfig};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CoreConfig {
    hosts_config: CommonHostsConfig,
    connection_config: ConnectionConfig,
    api_config: ApiConfig,
    db_config: DatabaseConfig,
    wallet_config: WalletConfig,
    did_config: DidConfig,
    issue_config: IssueConfig,
    verify_req_config: VerifyReqConfig,
    role: AuthorityRole,
    #[serde(default)]
    oauth: OauthConfig,
}

impl CoreConfig {
    pub fn load(env_file: String) -> Outcome<Self> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(env_file);
        debug!("Config file path: {}", path.display());

        let data = read(path)?;
        serde_norway::from_str(&data)
            .map_err(|e| Errors::parse("Unable to parse config file", Some(Box::new(e))))
    }
}

impl DatabaseConfigTrait for CoreConfig {
    fn db(&self) -> &DatabaseConfig {
        &self.db_config
    }
}

impl ApiConfigTrait for CoreConfig {
    fn api(&self) -> &ApiConfig {
        &self.api_config
    }
}

impl ConnectionConfigTrait for CoreConfig {
    fn connection(&self) -> &ConnectionConfig {
        &self.connection_config
    }
}

impl HostsConfigTrait for CoreConfig {
    fn hosts(&self) -> &CommonHostsConfig {
        &self.hosts_config
    }
}

impl DidConfigTrait for CoreConfig {
    fn did_config(&self) -> &DidConfig {
        &self.did_config
    }
}

impl IssueConfigTrait for CoreConfig {
    fn issue_config(&self) -> &IssueConfig {
        &self.issue_config
    }
}

impl VerifyReqConfigTrait for CoreConfig {
    fn verify_req_config(&self) -> &VerifyReqConfig {
        &self.verify_req_config
    }
}

impl WalletConfigTrait for CoreConfig {
    fn wallet_config(&self) -> &WalletConfig {
        &self.wallet_config
    }
}

impl RoleConfigTrait for CoreConfig {
    fn get_role(&self) -> &AuthorityRole {
        &self.role
    }
}

impl OauthConfigTrait for CoreConfig {
    fn oauth(&self) -> &OauthConfig {
        &self.oauth
    }
}

impl CoreConfigTrait for CoreConfig {}
