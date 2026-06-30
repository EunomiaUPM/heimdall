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

use super::super::VcBuilderTrait;
use crate::config::traits::RoleConfigTrait;
use crate::config::types::AuthorityRole;
use crate::services::vcs_builder::dataspace_authority::config::DataSpaceAuthorityConfig;
use crate::types::need_field_for_vc;
use ymir::data::entities::shared::issuance;
use ymir::errors::{Errors, Outcome};
use ymir::types::jwt::VCJwtClaims;
use ymir::types::vcs::vc_specs::dataspace::DataSpaceParticipant;
use ymir::types::vcs::VcTypeConfig;

pub struct DataSpaceAuthorityVcBuilder {
    config: DataSpaceAuthorityConfig,
}

impl DataSpaceAuthorityVcBuilder {
    pub fn new(config: DataSpaceAuthorityConfig) -> Self {
        Self { config }
    }
}

impl RoleConfigTrait for DataSpaceAuthorityVcBuilder {
    fn get_role(&self) -> &AuthorityRole {
        &self.config.get_role()
    }
}

impl VcBuilderTrait for DataSpaceAuthorityVcBuilder {
    fn build_vc(&self, model: &issuance::Model, vc_config: VcTypeConfig) -> Outcome<VCJwtClaims> {
        let holder_did = need_field_for_vc(model.build_ctx.holder_did.as_ref())?;

        let role = self.config.get_role();
        if !role.available_credentials().contains(vc_config.vc_type()) {
            return Err(Errors::unauthorized(
                format!("As a {} we cannot issue {}", role, vc_config),
                None,
            ));
        }

        let cred_sub = DataSpaceParticipant {
            id: holder_did.clone(),
            nickname: model.build_ctx.subject_name.clone(),
        };

        let credential_subject = serde_json::to_value(&cred_sub)?;
        self.just_build(&model, credential_subject, vc_config)
    }
}
