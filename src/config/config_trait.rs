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

use crate::types::enums::role::AuthorityRole;
use crate::types::secrets::DbSecrets;

pub trait CoreConfigTrait: Send + Sync + 'static {
    fn get_full_db(&self, db_secrets: DbSecrets) -> String;
    fn get_host(&self) -> String;
    fn is_local(&self) -> bool;
    fn get_weird_port(&self) -> String;
    fn get_role(&self) -> AuthorityRole;
    fn get_openapi_json(&self) -> anyhow::Result<String>;
    fn get_api_path(&self) -> String;
    fn is_wallet_active(&self) -> bool;
}
