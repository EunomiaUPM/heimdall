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
use serde_json::Value;
use std::str::FromStr;
use ymir::capabilities::DigestSRI;
use ymir::data::entities::shared::issuance;
use ymir::errors::{Errors, Outcome};
use ymir::types::crypto::Canon;
use ymir::types::jwt::{Jwt, VCJwtClaims};
use ymir::types::vcs::vc_specs::gx_label::{CompliantCredential, GxLabelCredSubject};
use ymir::types::vcs::{VcType, VcTypeConfig};

use super::super::VcBuilderTrait;
use super::ClearingHouseAuthorityConfig;
use crate::config::traits::{ClHConfigTrait, RoleConfigTrait};
use crate::config::types::AuthorityRole;
use crate::types::need_field_for_vc;

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
    fn build_vc(
        &self,
        issuance: &issuance::Model,
        vc_config: VcTypeConfig,
    ) -> Outcome<VCJwtClaims> {
        let holder_did = need_field_for_vc(issuance.build_ctx.holder_did.as_deref())?;

        let role = self.config.get_role();
        if !role.available_credentials().contains(vc_config.vc_type()) {
            return Err(Errors::unauthorized(
                format!("As a {} we cannot issue {}", role, vc_config),
                None,
            ));
        }

        let vcs = &issuance.build_ctx.vcs;
        if vcs.len() != 3 {
            return Err(Errors::unauthorized(
                "Not enough credentials were presented",
                None,
            ));
        }

        let vcs = &issuance.build_ctx.vcs;
        let mut compliant_credentials = Vec::with_capacity(vcs.len());

        let mut has_legal_person = false;
        let mut has_terms = false;
        let mut has_registration = false;

        for vc in vcs {
            let parsed = Self::parse_credential(vc)?;

            if let Ok(t) = VcType::from_str(&parsed.credential_type) {
                match t {
                    VcType::LegalPerson => has_legal_person = true,
                    VcType::TermsAndConditions => has_terms = true,
                    t if t.is_legal_registration_number() => has_registration = true,
                    _ => {}
                }
            }

            compliant_credentials.push(parsed);
        }

        if !has_legal_person {
            return Err(Errors::forbidden(
                "Missing required credential: LegalPerson",
                None,
            ));
        }
        if !has_terms {
            return Err(Errors::forbidden(
                "Missing required credential: TermsAndConditions",
                None,
            ));
        }
        if !has_registration {
            return Err(Errors::forbidden(
                "Missing required credential: any LegalRegistrationNumber (VatID, LeiCode, TaxID, ...)",
                None,
            ));
        }

        let cred_subj = GxLabelCredSubject {
            id: holder_did.to_string(),
            label_level: self.config.get_label_level().to_string(),
            engine_version: self.config.get_engine_version().to_string(),
            rules_version: self.config.get_rules_version().to_string(),
            compliant_credentials,
            validated_criteria: vec![self.config.get_validated_criteria().to_string()],
        };

        let credential_subject = serde_json::to_value(&cred_subj)?;
        self.just_build(&issuance, credential_subject, vc_config)
    }
}

impl ClearingHouseAuthorityVcBuilder {
    fn parse_credential(credential: &str) -> Outcome<CompliantCredential> {
        let jwt = Jwt::parse(credential)?;
        let claims_value: Value = jwt.unsafe_claims()?;
        let claims: VCJwtClaims = serde_json::from_value(claims_value.clone())?;
        let doc = &claims.vc_doc().r#type;

        let vc_type = doc
            .iter()
            .find(|t| t.as_str() != "VerifiableCredential")
            .ok_or_else(|| Errors::unauthorized("Credential type missing", None))?;

        let canon = Canon::try_from(&claims_value)?;
        Ok(CompliantCredential {
            credential_type: vc_type.clone(),
            digest_sri: DigestSRI::digest(&canon),
        })
    }
}
