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

use ymir::config::traits::ApiConfigTrait;
use ymir::config::types::CommonHostsConfig;

use super::GnapConfigTrait;
use crate::config::{CoreApplicationConfig, CoreConfigTrait};
use crate::types::role::AuthorityRole;

pub struct GnapConfig {
    hosts: CommonHostsConfig,
    role: AuthorityRole,
    api_path: String,
    is_cert_allowed: bool
}

impl From<CoreApplicationConfig> for GnapConfig {
    fn from(config: CoreApplicationConfig) -> GnapConfig {
        let api_path = config.get_api_version();
        let is_cert_allowed = config.requirements_to_verify.is_cert_allowed;
        GnapConfig { hosts: config.hosts().clone(), role: config.role, api_path, is_cert_allowed }
    }
}

impl GnapConfigTrait for GnapConfig {
    fn hosts(&self) -> &CommonHostsConfig { &self.hosts }
    fn get_role(&self) -> &AuthorityRole { &self.role }
    fn get_api_path(&self) -> String { self.api_path.clone() }
    fn is_cert_allowed(&self) -> bool { self.is_cert_allowed }
}
