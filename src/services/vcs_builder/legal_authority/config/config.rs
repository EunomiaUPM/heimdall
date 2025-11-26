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
use crate::config::CoreApplicationConfig;
use super::config_trait::LegalAuthorityConfigTrait;
use crate::types::enums::data_model::VcDataModelVersion;

pub struct LegalAuthorityConfig {
    vc_data_model: VcDataModelVersion,
}

impl LegalAuthorityConfigTrait for LegalAuthorityConfig {
    fn get_data_model(&self) -> &VcDataModelVersion {
        &self.vc_data_model
    }
}

impl From<CoreApplicationConfig> for LegalAuthorityConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        Self {
            vc_data_model: value.vc_data_model,
        }
    }
}

