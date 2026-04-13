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

use ymir::config::traits::VcConfigTrait;
use ymir::config::types::VcConfig;

use crate::config::traits::{DSConfigTrait, IssueConfigTrait, RoleConfigTrait};
use crate::config::types::{AuthorityRole, DsBuilderConfig};
use crate::config::CoreApplicationConfig;
use crate::services::vcs_builder::BuilderConfigDefaultTrait;

pub struct DataSpaceAuthorityConfig {
    vc_config: VcConfig,
    ds_config: DsBuilderConfig,
    role: AuthorityRole,
}

impl VcConfigTrait for DataSpaceAuthorityConfig {
    fn vc_config(&self) -> &VcConfig {
        &self.vc_config
    }
}

impl RoleConfigTrait for DataSpaceAuthorityConfig {
    fn get_role(&self) -> &AuthorityRole {
        &self.role
    }
}

impl BuilderConfigDefaultTrait for DataSpaceAuthorityConfig {}

impl DSConfigTrait for DataSpaceAuthorityConfig {
    fn get_ds_config(&self) -> &DsBuilderConfig {
        &self.ds_config
    }
}

impl From<CoreApplicationConfig> for DataSpaceAuthorityConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        Self {
            vc_config: value.vc_config().clone(),
            ds_config: value.get_ds_config().clone(),
            role: value.get_role().clone(),
        }
    }
}
