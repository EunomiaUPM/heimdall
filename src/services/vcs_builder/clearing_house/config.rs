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

use crate::config::traits::{ClHConfigTrait, IssueConfigTrait, RoleConfigTrait};
use crate::config::types::{AuthorityRole, ClHBuilderConfig};
use crate::config::CoreApplicationConfig;
use crate::services::vcs_builder::BuilderConfigDefaultTrait;

pub struct ClearingHouseAuthorityConfig {
    clh_config: ClHBuilderConfig,
    role: AuthorityRole,
}


impl RoleConfigTrait for ClearingHouseAuthorityConfig {
    fn get_role(&self) -> &AuthorityRole {
        &self.role
    }
}

impl BuilderConfigDefaultTrait for ClearingHouseAuthorityConfig {}

impl ClHConfigTrait for ClearingHouseAuthorityConfig {
    fn get_clh_config(&self) -> &ClHBuilderConfig {
        &self.clh_config
    }
}

impl From<CoreApplicationConfig> for ClearingHouseAuthorityConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        Self {
            clh_config: value.get_clh_config().clone(),
            role: value.get_role().clone(),
        }
    }
}
