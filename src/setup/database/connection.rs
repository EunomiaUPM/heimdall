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

use sea_orm::{Database, DatabaseConnection};

use crate::config::{CoreApplicationConfig, CoreConfigTrait};
use crate::services::vault::vault_rs::VaultService;
use crate::services::vault::VaultTrait;
use crate::types::secrets::DbSecrets;
use crate::utils::expect_from_env;

pub struct DbConnector {}

impl DbConnector {
    pub async fn get_connection(config: &CoreApplicationConfig) -> DatabaseConnection {
        let vault = VaultService::new();

        let db_path = expect_from_env("VAULT_DB");

        let db_secrets: DbSecrets =
            vault.read(None, &db_path).await.expect("Not able to retrieve env files");
        Database::connect(config.get_full_db(db_secrets)).await.expect("Database can't connect")
    }
}
