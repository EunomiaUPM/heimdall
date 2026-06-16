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
use ymir::capabilities::DigestSRI;
use ymir::data::entities::{issuing, vc_request};
use ymir::errors::{BadFormat, Errors, Outcome};
use ymir::types::crypto::Canon;
use ymir::types::jwt::{Jwt, VCJwtClaims, VPJwtClaims};
use ymir::types::present::{Missing, Present};
use ymir::types::vcs::vc_specs::gx_label::{CompliantCredential, GxLabelCredSubjectBuilder};
use ymir::types::vcs::VcType;

use super::super::VcBuilderTrait;
use super::ClearingHouseAuthorityConfig;
use crate::config::traits::{ClHConfigTrait, RoleConfigTrait};
use crate::config::types::AuthorityRole;

pub struct ClearingHouseAuthorityVcBuilder {
    config: ClearingHouseAuthorityConfig,
}

impl ClearingHouseAuthorityVcBuilder {
    pub fn new(config: ClearingHouseAuthorityConfig) -> Self {
        Self { config }
    }
}

impl RoleConfigTrait for ClearingHouseAuthorityVcBuilder {
    fn get_role(&self) -> &AuthorityRole {
        &self.config.get_role()
    }
}

impl VcBuilderTrait for ClearingHouseAuthorityVcBuilder {
    fn build_vc(&self, model: &issuing::Model) -> Outcome<Value> {
        let vc_type = VcType::from_str(&model.vc_type)?;

        if !matches!(vc_type, VcType::GxLabel) {
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

        let vc = serde_json::from_str::<
            GxLabelCredSubjectBuilder<Missing, Present, Present, Present>,
        >(vc_data)?;

        let cred_subj = vc.id(holder_did).build();

        let credential_subject = serde_json::to_value(&cred_subj)?;
        self.just_build(&model, credential_subject, &self.config)
    }

    fn gather_data(&self, req_model: &vc_request::Model) -> Outcome<String> {
        let builder = GxLabelCredSubjectBuilder::new(
            self.config.get_label_level(),
            self.config.get_engine_version(),
            self.config.get_rules_version(),
            self.config.get_validated_criteria(),
        );

        let vpt = req_model.vpt.as_ref().ok_or_else(|| {
            Errors::unauthorized("Authentication has not been completed yet", None)
        })?;

        let data = Self::complete(vpt, builder)?;

        Ok(serde_json::to_string(&data)?)
    }

    fn validate(&self, vc_type: &str) -> Outcome<VcType> {
        let vc_type = VcType::from_str(vc_type)?;

        match &vc_type {
            VcType::GxLabel => Ok(vc_type),
            vc_type => Err(Errors::unauthorized(
                format!("Unauthorized to issue vc_type {}", vc_type.to_string()),
                None,
            )),
        }
    }
}

impl ClearingHouseAuthorityVcBuilder {
    fn parse_credential(credential: &str) -> Outcome<CompliantCredential> {
        let credential: VCJwtClaims = serde_json::from_str(credential)?;

        let doc = credential.vc_doc();
        let id = &doc.id.clone();

        let vc_type = credential
            .vc_doc()
            .r#type
            .iter()
            .find(|v| v.as_str() != "VerifiableCredential")
            .ok_or(Errors::format(
                BadFormat::Received,
                "No VC type found other than VerifiableCredential",
                None,
            ))?
            .clone();

        let value = serde_json::to_value(credential)?;
        let canon = Canon::try_from(&value)?;
        let digest_sri = DigestSRI::digest(&canon);

        Ok(CompliantCredential {
            id: id.clone(),
            r#type: vc_type.to_string(),
            digest_sri,
        })
    }

    pub fn complete(
        vpt: &str,
        builder: GxLabelCredSubjectBuilder<Missing, Missing, Missing, Missing>,
    ) -> Outcome<GxLabelCredSubjectBuilder<Missing, Present, Present, Present>> {
        let vp = Jwt::parse(vpt)?;
        let claims: VPJwtClaims = vp.unsafe_claims()?;

        let credentials = claims.vp.verifiable_credential;

        if credentials.len() != 3 {
            return Err(Errors::unauthorized(
                "Did not send the correct amount of credentials",
                None,
            ));
        }

        let legal_person = Self::parse_credential(&credentials[0])?;
        let reg_number = Self::parse_credential(&credentials[1])?;
        let terms_cons = Self::parse_credential(&credentials[2])?;

        Ok(builder
            .legal_person(legal_person)
            .reg_number(reg_number)
            .terms_cons(terms_cons))
    }
}
