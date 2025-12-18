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

use super::CoreConfigTrait;
use crate::setup::database::{DatabaseConfig, DbConnectionTrait, DbType};
use crate::types::api::ApiConfig;
use crate::types::enums::data_model::W3cDataModelVersion;
use crate::types::enums::role::AuthorityRole;
use crate::types::host::{HostConfig, HostConfigTrait};
use crate::types::issuing::{StuffToIssue, VcModel};
use crate::types::secrets::DbSecrets;
use crate::types::verifying::RequirementsToVerify;
use crate::types::wallet::WalletConfig;
use crate::utils::read;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CoreApplicationConfig {
    pub host: HostConfig,
    pub is_local: bool,
    pub db_config: DatabaseConfig,
    pub wallet_config: Option<WalletConfig>,
    pub role: AuthorityRole,
    pub api: ApiConfig,
    pub stuff_to_issue: StuffToIssue,
    pub requirements_to_verify: RequirementsToVerify
}

impl Default for CoreApplicationConfig {
    fn default() -> Self {
        Self {
            host: HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: Some("1500".to_string())
            },
            db_config: DatabaseConfig {
                r#type: DbType::Postgres,
                url: "127.0.0.1".to_string(),
                port: "1450".to_string()
            },
            wallet_config: Some(WalletConfig {
                api: HostConfig {
                    protocol: "http".to_string(),
                    url: "127.0.0.1".to_string(),
                    port: Some("7001".to_string())
                },
                id: None
            }),
            is_local: true,
            api: ApiConfig {
                version: "v1".to_string(),
                openapi_path: "static/specs/openapi/openapi.json".to_string()
            },
            role: AuthorityRole::LegalAuthority,
            requirements_to_verify: RequirementsToVerify {
                is_cert_allowed: true,
                vcs_requested: vec![]
            },
            stuff_to_issue: StuffToIssue {
                vc_model: VcModel::JwtVc,
                w3c_data_model: Some(W3cDataModelVersion::V2),
                dataspace_id: None,
                federated_catalog_uri: None
            }
        }
    }
}

impl CoreApplicationConfig {
    pub fn load(env_file: Option<String>) -> Self {
        if let Some(env_file) = env_file {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(env_file);
            debug!("Config file path: {}", path.display());

            let data = fs::read_to_string(&path).expect("Unable to read config file");
            serde_norway::from_str(&data).expect("Unable to parse config file")
        } else {
            CoreApplicationConfig::default()
        }
    }
}

impl CoreConfigTrait for CoreApplicationConfig {
    fn get_full_db(&self, db_secrets: DbSecrets) -> String {
        self.db_config.get_full_db(db_secrets)
    }
    fn get_host(&self) -> String { self.host.get_host() }

    fn is_local(&self) -> bool { self.is_local }

    fn get_weird_port(&self) -> String {
        let host = self.host.clone();
        match host.port {
            Some(data) => {
                format!(":{}", data)
            }
            None => "".to_string()
        }
    }
    fn get_role(&self) -> AuthorityRole { self.role.clone() }

    fn get_openapi_json(&self) -> anyhow::Result<String> { read(&self.api.openapi_path) }
    fn get_api_path(&self) -> String { format!("/api/{}", self.api.version) }
    fn is_wallet_active(&self) -> bool { self.wallet_config.is_some() }
}
