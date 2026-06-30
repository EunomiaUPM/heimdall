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

use std::sync::Arc;

use ymir::data::entities::shared::issuance;
use ymir::errors::{Errors, Outcome};
use ymir::types::jwt::VCJwtClaims;
use ymir::types::vcs::{VcType, VcTypeConfig};

use crate::config::traits::RoleConfigTrait;
use crate::config::types::AuthorityRole;
use crate::services::vcs_builder::clearing_house::ClearingHouseAuthorityVcBuilder;
use crate::services::vcs_builder::dataspace_authority::DataSpaceAuthorityVcBuilder;
use crate::services::vcs_builder::legal_authority::LegalAuthorityVcBuilder;
use crate::services::vcs_builder::VcBuilderTrait;

pub struct EcoAuthorityBuilder {
    legal: Arc<LegalAuthorityVcBuilder>,
    dataspace: Arc<DataSpaceAuthorityVcBuilder>,
    clearing_house: Arc<ClearingHouseAuthorityVcBuilder>,
}

impl EcoAuthorityBuilder {
    pub fn new(
        legal: Arc<LegalAuthorityVcBuilder>,
        dataspace: Arc<DataSpaceAuthorityVcBuilder>,
        clearing_house: Arc<ClearingHouseAuthorityVcBuilder>,
    ) -> Self {
        Self {
            legal,
            dataspace,
            clearing_house,
        }
    }
}

impl RoleConfigTrait for EcoAuthorityBuilder {
    fn get_role(&self) -> &AuthorityRole {
        &AuthorityRole::EcoAuthority
    }
}

impl VcBuilderTrait for EcoAuthorityBuilder {
    fn build_vc(&self, model: &issuance::Model, vc_config: VcTypeConfig) -> Outcome<VCJwtClaims> {
        match vc_config.vc_type() {
            VcType::Eori
            | VcType::Euid
            | VcType::LocalRegistrationNumber
            | VcType::LeiCode
            | VcType::VatId
            | VcType::TaxId => self.legal.build_vc(&model, vc_config),
            VcType::DataspaceParticipant => self.dataspace.build_vc(model, vc_config),
            VcType::GxLabel => self.clearing_house.build_vc(model, vc_config),
            _ => Err(Errors::unauthorized(
                format!("Cannot issue vc type: {}", vc_config.vc_type()),
                None,
            )),
        }
    }
}
