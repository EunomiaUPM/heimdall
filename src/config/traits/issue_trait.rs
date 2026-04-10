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

use crate::config::types::{ClHBuilderConfig, DsBuilderConfig, IssueConfig};

pub trait IssueConfigTrait {
    fn issue_config(&self) -> &IssueConfig;
    fn get_ds_config(&self) -> &DsBuilderConfig {
        &self
            .issue_config()
            .ds_config
            .as_ref()
            .expect("Requested dataspace issue configuration is not defined, and wants to be used")
    }
    fn get_clh_config(&self) -> &ClHBuilderConfig {
        &self.issue_config().clh_config.as_ref().expect(
            "Requested clearing house issue configuration is not defined, and wants to be used",
        )
    }
}
