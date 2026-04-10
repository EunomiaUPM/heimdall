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

use crate::config::types::ClHBuilderConfig;

pub trait ClHConfigTrait {
    fn get_clh_config(&self) -> &ClHBuilderConfig;
    fn get_label_level(&self) -> &str {
        &self.get_clh_config().label_level
    }
    fn get_engine_version(&self) -> &str {
        &self.get_clh_config().engine_version
    }
    fn get_rules_version(&self) -> &str {
        &self.get_clh_config().rules_version
    }
    fn get_validated_criteria(&self) -> &str {
        &self.get_clh_config().validated_criteria
    }
}
