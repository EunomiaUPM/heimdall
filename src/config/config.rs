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

use serde::{Deserialize, Serialize};
use ymir::config::traits::{
    ApiConfigTrait, CommonConfigTrait, ConfigLoader, ConnectionConfigTrait, DatabaseConfigTrait,
    DidConfigTrait, HostsConfigTrait, OauthConfigTrait, VerifyReqConfigTrait, WalletConfigTrait,
};
use ymir::config::types::{
    ApiConfig, CommonConfig, CommonHostsConfig, ConnectionConfig, DatabaseConfig, DidConfig,
    OauthConfig, VerifyReqConfig, WalletConfig,
};

use super::CoreConfigTrait;
use crate::config::traits::{IssueConfigTrait, Oid4VpServiceTrait, RoleConfigTrait};
use crate::config::types::{AuthorityRole, IssueConfig, Oid4VpService};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CoreConfig {
    /// `hosts`, `db`, `api`, `connection` and `oauth`, shared with every agent.
    #[serde(flatten)]
    common: CommonConfig,
    wallet_config: WalletConfig,
    did_config: DidConfig,
    issue_config: IssueConfig,
    verify_req_config: VerifyReqConfig,
    oid4vp_service: Oid4VpService,
    role: AuthorityRole,
}

impl ConfigLoader for CoreConfig {}

impl CommonConfigTrait for CoreConfig {
    fn common(&self) -> &CommonConfig {
        &self.common
    }
}

impl DatabaseConfigTrait for CoreConfig {
    fn db(&self) -> &DatabaseConfig {
        self.common.db()
    }
}

impl ApiConfigTrait for CoreConfig {
    fn api(&self) -> &ApiConfig {
        self.common.api()
    }
}

impl ConnectionConfigTrait for CoreConfig {
    fn connection(&self) -> &ConnectionConfig {
        self.common.connection()
    }
}

impl HostsConfigTrait for CoreConfig {
    fn hosts(&self) -> &CommonHostsConfig {
        self.common.hosts()
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
        self.common.oauth()
    }
}

impl Oid4VpServiceTrait for CoreConfig {
    fn oid4vp_service(&self) -> &Oid4VpService {
        &self.oid4vp_service
    }
}

impl CoreConfigTrait for CoreConfig {}
