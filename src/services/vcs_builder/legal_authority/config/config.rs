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

use crate::config::CoreApplicationConfig;
use crate::services::vcs_builder::min_builder_config::MinConfig;
use crate::services::vcs_builder::ConfigMinTrait;
use crate::types::enums::data_model::W3cDataModelVersion;
use crate::types::issuing::VcModel;

pub struct LegalAuthorityConfig {
    min_config: MinConfig
}

impl ConfigMinTrait for LegalAuthorityConfig {
    fn get_vc_model(&self) -> &VcModel { self.min_config.get_vc_model() }

    fn get_w3c_data_model(&self) -> &Option<W3cDataModelVersion> {
        self.min_config.get_w3c_data_model()
    }
}

impl From<CoreApplicationConfig> for LegalAuthorityConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        Self {
            min_config: MinConfig {
                vc_model: value.stuff_to_issue.vc_model,
                vc_data_model: value.stuff_to_issue.w3c_data_model
            }
        }
    }
}
