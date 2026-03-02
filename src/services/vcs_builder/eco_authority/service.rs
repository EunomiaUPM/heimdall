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

use std::str::FromStr;
use std::sync::Arc;

use serde_json::Value;
use ymir::data::entities::{issuing, vc_request};
use ymir::errors::{Errors, Outcome};
use ymir::types::vcs::VcType;

use crate::config::role::{AuthorityRole, RoleConfigTrait};
use crate::services::vcs_builder::dataspace_authority::DataSpaceAuthorityVcBuilder;
use crate::services::vcs_builder::legal_authority::LegalAuthorityVcBuilder;
use crate::services::vcs_builder::VcBuilderTrait;

pub struct EcoAuthorityBuilder {
    legal: Arc<LegalAuthorityVcBuilder>,
    dataspace: Arc<DataSpaceAuthorityVcBuilder>
}

impl EcoAuthorityBuilder {
    pub fn new(
        legal: Arc<LegalAuthorityVcBuilder>,
        dataspace: Arc<DataSpaceAuthorityVcBuilder>
    ) -> Self {
        Self { legal, dataspace }
    }
}

impl RoleConfigTrait for EcoAuthorityBuilder {
    fn get_role(&self) -> &AuthorityRole { &AuthorityRole::EcoAuthority }
}

impl VcBuilderTrait for EcoAuthorityBuilder {
    fn build_vc(&self, model: &issuing::Model) -> Outcome<Value> {
        let vc_type = VcType::from_str(&model.vc_type)?;
        match vc_type {
            VcType::LegalRegistrationNumber(_) => self.legal.build_vc(model),
            VcType::DataspaceParticipant => self.dataspace.build_vc(model),
            _ => Err(Errors::unauthorized(
                format!("Cannot issue vc type: {}", vc_type),
                None
            ))
        }
    }

    fn gather_data(&self, req_model: &vc_request::Model) -> Outcome<String> {
        let vc_type = VcType::from_str(&req_model.vc_type)?;
        match vc_type {
            VcType::LegalRegistrationNumber(_) => self.legal.gather_data(&req_model),
            VcType::DataspaceParticipant => self.dataspace.gather_data(&req_model),
            _ => Err(Errors::unauthorized(
                format!("Cannot issue vc type: {}", vc_type),
                None
            ))
        }
    }
}
