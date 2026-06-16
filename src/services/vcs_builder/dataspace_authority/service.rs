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

use std::str::FromStr;

use serde_json::Value;
use tracing::info;
use ymir::data::entities::{issuing, vc_request};
use ymir::errors::{Errors, Outcome};
use ymir::types::present::Missing;
use ymir::types::vcs::vc_specs::dataspace::DataSpaceParticipantBuilder;
use ymir::types::vcs::VcType;

use super::super::VcBuilderTrait;
use crate::config::traits::RoleConfigTrait;
use crate::config::types::AuthorityRole;
use crate::services::vcs_builder::dataspace_authority::config::DataSpaceAuthorityConfig;

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
    fn build_vc(&self, model: &issuing::Model) -> Outcome<Value> {
        let vc_type = VcType::from_str(&model.vc_type)?;

        if !matches!(vc_type, VcType::DataspaceParticipant) {
            return Err(Errors::unauthorized(
                format!("Cannot issue vc type: {}", vc_type),
                None,
            ));
        }

        info!("Building {} credential", vc_type);

        let holder_did = model.holder_did.as_ref().ok_or_else(|| {
            Errors::missing_resource("holder did", "Missing holder did in db", None)
        })?;
        let vc_data = model
            .credential_data
            .as_deref()
            .ok_or_else(|| Errors::crazy("Tried to issue a credential without any data", None))?;

        let vc = serde_json::from_str::<DataSpaceParticipantBuilder<Missing>>(vc_data)?;

        let cred_subj = vc.id(holder_did).build();

        let credential_subject = serde_json::to_value(&cred_subj)?;
        self.just_build(&model, credential_subject, &self.config)
    }

    fn gather_data(&self, req_model: &vc_request::Model) -> Outcome<String> {
        let data = DataSpaceParticipantBuilder::new(req_model.participant_slug.clone());
        Ok(serde_json::to_string(&data)?)
    }

    fn validate(&self, vc_type: &str) -> Outcome<VcType> {
        let vc_type = VcType::from_str(vc_type)?;

        match &vc_type {
            VcType::DataspaceParticipant => Ok(vc_type),
            vc_type => Err(Errors::unauthorized(
                format!("Unauthorized to issue vc_type {}", vc_type.to_string()),
                None,
            )),
        }
    }
}
