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
use ymir::types::vcs::{VcModel, W3cDataModelVersion};

use crate::config::{CoreApplicationConfig, CoreConfigTrait};
use crate::services::vcs_builder::BuilderConfigDefaultTrait;

pub struct LegalAuthorityConfig {
    vc_config: VcConfig,
}

impl BuilderConfigDefaultTrait for LegalAuthorityConfig {
    fn get_vc_model(&self) -> &VcModel {
        self.vc_config.get_vc_model()
    }

    fn get_w3c_data_model(&self) -> Option<&W3cDataModelVersion> {
        self.vc_config.get_w3c_data_model()
    }
}

impl From<CoreApplicationConfig> for LegalAuthorityConfig {
    fn from(value: CoreApplicationConfig) -> Self {
        Self {
            vc_config: VcConfig {
                vc_model: value.get_vc_config().get_vc_model().clone(),
                w3c_data_model: value.get_vc_config().get_w3c_data_model().cloned(),
            },
        }
    }
}
