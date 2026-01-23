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

use super::helper::DbType;
use crate::setup::database::DatabaseConfig;
use crate::types::secrets::DbSecrets;

pub trait DbConnectionTrait: Send + Sync + 'static {
    fn get_raw_database_config(&self) -> &DatabaseConfig;
    fn get_full_db(&self, db_secrets: DbSecrets) -> String {
        let db_config = self.get_raw_database_config();
        match db_config.r#type {
            DbType::Memory => ":memory:".to_string(),
            _ => format!(
                "{}://{}:{}@{}:{}/{}",
                db_config.r#type,
                db_secrets.user,
                db_secrets.password,
                db_config.url,
                db_config.port,
                db_secrets.name
            )
        }
    }
}
