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

use ymir::config::traits::VcConfigTrait;
use ymir::config::types::VcConfig;

use crate::config::role::{AuthorityRole, RoleConfigTrait};
use crate::config::CoreApplicationConfig;
use crate::services::vcs_builder::BuilderConfigDefaultTrait;

pub struct LegalAuthorityConfig {
    vc_config: VcConfig,
    role: AuthorityRole
}

impl VcConfigTrait for LegalAuthorityConfig {
    fn vc_config(&self) -> &VcConfig { &self.vc_config }
}

impl RoleConfigTrait for LegalAuthorityConfig {
    fn get_role(&self) -> &AuthorityRole { &self.role }
}

impl BuilderConfigDefaultTrait for LegalAuthorityConfig {}

impl From<CoreApplicationConfig> for LegalAuthorityConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        Self { vc_config: value.vc_config().clone(), role: value.get_role().clone() }
    }
}
