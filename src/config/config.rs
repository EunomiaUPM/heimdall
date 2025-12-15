/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use super::CoreApplicationConfigTrait;
use crate::setup::database::{DatabaseConfig, DbType};
use crate::types::api::ApiConfig;
use crate::types::enums::data_model::W3cDataModelVersion;
use crate::types::enums::role::AuthorityRole;
use crate::types::host::HostConfig;
use crate::types::verifying::RequirementsToVerify;
use crate::types::wallet::WalletConfig;
use crate::utils::read;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};
use tracing::debug;
use crate::types::issuing::{StuffToIssue, VcModel};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CoreApplicationConfig {
    pub host: HostConfig,
    pub is_local: bool,
    pub database_config: DatabaseConfig,
    pub wallet_config: WalletConfig,
    pub role: AuthorityRole,
    pub keys_path: String,
    pub api: ApiConfig,
    pub stuff_to_issue: StuffToIssue,
    pub requirements_to_verify: RequirementsToVerify,
}

impl Default for CoreApplicationConfig {
    fn default() -> Self {
        Self {
            host: HostConfig {
                protocol: "http".to_string(),
                url: "127.0.0.1".to_string(),
                port: Some("1500".to_string()),
            },
            database_config: DatabaseConfig {
                r#type: DbType::Postgres,
                url: "127.0.0.1".to_string(),
                port: "1450".to_string(),
                user: "ds_authority".to_string(),
                password: "ds_authority".to_string(),
                name: "ds_authority".to_string(),
            },
            wallet_config: WalletConfig {
                api_protocol: "http".to_string(),
                api_url: "127.0.0.1".to_string(),
                api_port: Some("7001".to_string()),
                r#type: "email".to_string(),
                name: "RainbowAuthority".to_string(),
                email: "RainbowAuthority@rainbow.com".to_string(),
                password: "rainbow".to_string(),
                id: None,
            },
            is_local: true,
            keys_path: "static/certificates/".to_string(),
            api: ApiConfig {
                version: "v1".to_string(),
                openapi_path: "static/specs/openapi/openapi.json".to_string(),
            },
            role: AuthorityRole::LegalAuthority,
            requirements_to_verify: RequirementsToVerify {
                is_cert_allowed: true,
                vcs_requested: vec![],
            },
            stuff_to_issue: StuffToIssue {
                vc_model: VcModel::JwtVc,
                w3c_data_model: Some(W3cDataModelVersion::V2),
                dataspace_id: Some("rainbow_authority".to_string()),
            },
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

impl CoreApplicationConfigTrait for CoreApplicationConfig {
    fn get_full_db_url(&self) -> String {
        let db_config = self.get_raw_database_config();
        match db_config.r#type {
            DbType::Memory => ":memory:".to_string(),
            _ => format!(
                "{}://{}:{}@{}:{}/{}",
                db_config.r#type,
                db_config.user,
                db_config.password,
                db_config.url,
                db_config.port,
                db_config.name
            ),
        }
    }

    fn get_raw_database_config(&self) -> &DatabaseConfig {
        &self.database_config
    }

    fn get_host(&self) -> String {
        let host = self.host.clone();
        match host.port {
            Some(port) => {
                format!("{}://{}:{}", host.protocol, host.url, port)
            }
            None => {
                format!("{}://{}", host.protocol, host.url)
            }
        }
    }

    fn is_local(&self) -> bool {
        self.is_local
    }

    fn get_weird_port(&self) -> String {
        let host = self.host.clone();
        match host.port {
            Some(data) => {
                format!(":{}", data)
            }
            None => "".to_string(),
        }
    }
    fn get_role(&self) -> AuthorityRole {
        self.role.clone()
    }

    fn get_openapi_json(&self) -> anyhow::Result<String> {
        read(&self.api.openapi_path)
    }
    fn get_api_path(&self) -> String {
        format!("/api/{}", self.api.version)
    }
}
