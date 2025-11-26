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
use crate::types::enums::data_model::VcDataModelVersion;
use crate::types::enums::role::AuthorityRole;
use crate::types::enums::vc_type::VcType;
use crate::types::host::HostConfig;
use crate::types::wallet::WalletConfig;
use crate::utils::read;
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use std::{env, fs};
use tracing::debug;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CoreApplicationConfig {
    pub host: HostConfig,
    pub is_local: bool,
    pub database_config: DatabaseConfig,
    pub wallet_config: WalletConfig,
    pub vc_data_model: VcDataModelVersion,
    pub role: AuthorityRole,
    #[serde(deserialize_with = "deserialize_vctype_vec")]
    pub requested_vcs: Vec<VcType>,
    pub keys_path: String,
    pub api: ApiConfig,
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
            vc_data_model: VcDataModelVersion::V1,
            role: AuthorityRole::LegalAuthority,
            requested_vcs: vec![],
        }
    }
}

impl CoreApplicationConfig {
    pub fn load_from_yaml(env_file: Option<String>) -> Self {
        if let Some(env_file) = env_file {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(env_file);
            debug!("Config file path: {}", path.display());

            let data = fs::read_to_string(&path).expect("Unable to read config file");
            serde_yaml::from_str(&data).expect("Unable to parse config file")
        } else {
            CoreApplicationConfig::default()
        }
    }

    pub fn merge_dotenv_configuration(env_file: Option<String>) -> Self {
        if let Some(env_file) = env_file {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(env_file.clone());

            debug!("Config file path: {}", path.display());
            dotenvy::from_filename(path).expect(".env not found");
        }

        dotenvy::dotenv().ok();
        let default = CoreApplicationConfig::default();
        let compound_config = Self {
            host: HostConfig {
                protocol: extract_env("HOST_PROTOCOL", default.host.clone().protocol),
                url: extract_env("HOST_URL", default.host.clone().url),
                port: option_extract_env("HOST_PORT"),
            },
            database_config: DatabaseConfig {
                r#type: extract_env("DB_TYPE", default.database_config.r#type.to_string())
                    .parse()
                    .unwrap(),
                url: extract_env("DB_URL", default.database_config.url),
                port: extract_env("DB_PORT", default.database_config.port),
                user: extract_env("DB_USER", default.database_config.user),
                password: extract_env("DB_PASSWORD", default.database_config.password),
                name: extract_env("DB_DATABASE", default.database_config.name),
            },
            wallet_config: WalletConfig {
                api_protocol: extract_env(
                    "WALLET_API_PROTOCOL",
                    default.wallet_config.api_protocol,
                ),
                api_url: extract_env("WALLET_API_URL", default.wallet_config.api_url),
                api_port: option_extract_env("WALLET_API_PORT"),
                r#type: extract_env("WALLET_TYPE", default.wallet_config.r#type),
                name: extract_env("WALLET_NAME", default.wallet_config.name),
                email: extract_env("WALLET_EMAIL", default.wallet_config.email),
                password: extract_env("WALLET_PASSWORD", default.wallet_config.password),
                id: None,
            },
            vc_data_model: VcDataModelVersion::from_str(&extract_env(
                "VC_DATA_MODEL",
                default.vc_data_model.to_string(),
            ))
            .expect("Invalid_version"),
            role: AuthorityRole::from_str(&extract_env("ROLE", default.role.to_string()))
                .expect("InvalidRole"),
            requested_vcs: serde_json::from_str(&extract_env(
                "REQUESTED_VCS",
                serde_json::to_string(&default.requested_vcs).unwrap(),
            ))
            .expect("Invalid requested credentials"),
            keys_path: extract_env("KEYS_PATH", default.keys_path),
            is_local: extract_env("IS_LOCAL", default.is_local.to_string())
                .parse()
                .unwrap(),
            api: ApiConfig {
                version: extract_env("API_VERSION", default.api.version),
                openapi_path: extract_env("OPENAPI_PATH", default.api.openapi_path),
            },
        };

        compound_config
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
    fn get_requested_vcs(&self) -> Vec<VcType> {
        self.requested_vcs.clone()
    }
    fn get_openapi_json(&self) -> anyhow::Result<String> {
        read(&self.api.openapi_path)
    }
    fn get_api_path(&self) -> String {
        format!("/api/{}", self.api.version)
    }
}

fn extract_env(env_var_name: &str, default: String) -> String {
    env::var(env_var_name).unwrap_or(default)
}

fn option_extract_env(env_var_name: &str) -> Option<String> {
    match env::var(env_var_name) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn deserialize_vctype_vec<'de, D>(deserializer: D) -> Result<Vec<VcType>, D::Error>
where
    D: Deserializer<'de>,
{
    let strings: Vec<String> = Vec::deserialize(deserializer)?;
    strings
        .into_iter()
        .map(|s| s.parse::<VcType>().map_err(de::Error::custom))
        .collect()
}
