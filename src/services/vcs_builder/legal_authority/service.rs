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

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde_json::Value;
use tracing::info;
use x509_parser::parse_x509_certificate;
use ymir::data::entities::{issuing, vc_request};
use ymir::errors::{BadFormat, Errors, Outcome};
use ymir::types::vcs::vc_specs::legal_authority::{
    LegalRegistrationNumberCredSubj, LegalRegistrationNumberTypes, VCData
};
use ymir::types::vcs::VcType;
use ymir::utils::{get_from_opt, parse_from_str, parse_to_string, parse_to_value};

use super::super::VcBuilderTrait;
use crate::config::role::{AuthorityRole, RoleConfigTrait};
use crate::services::vcs_builder::legal_authority::config::LegalAuthorityConfig;

pub struct LegalAuthorityVcBuilder {
    config: LegalAuthorityConfig
}

impl LegalAuthorityVcBuilder {
    pub fn new(config: LegalAuthorityConfig) -> Self { Self { config } }
}

impl RoleConfigTrait for LegalAuthorityVcBuilder {
    fn get_role(&self) -> &AuthorityRole { &self.config.get_role() }
}

impl VcBuilderTrait for LegalAuthorityVcBuilder {
    fn build_vc(&self, model: &issuing::Model) -> Outcome<Value> {
        let vc_type = VcType::from_str(&model.vc_type)?;
        info!("Building {} credential", vc_type);

        let vc_data: VCData =
            parse_from_str(&get_from_opt(model.credential_data.as_ref(), "credential data")?)?;
        let holder_did = get_from_opt(model.holder_did.as_ref(), "holder did")?;

        let VcType::LegalRegistrationNumber(data) = vc_type else {
            return Err(Errors::unauthorized(
                format!("Cannot issue vc type: {}", vc_type),
                None
            ));
        };

        let cred_subj =
            LegalRegistrationNumberCredSubj::new(data, &holder_did, &vc_data.shitty_code);

        let credential_subject = parse_to_value(&cred_subj)?;

        self.just_build(&model, credential_subject, &self.config)
    }

    fn gather_data(&self, req_model: &vc_request::Model) -> Outcome<String> {
        info!("Gathering data to issue vc");

        let base_cert = req_model.cert.as_ref().ok_or_else(|| {
            Errors::format(BadFormat::Received, "There was no cert in the Grant Request", None)
        })?;

        let cert_bytes = STANDARD.decode(base_cert).map_err(|e| {
            Errors::format(BadFormat::Received, "Unable to decode certificate", Some(Box::new(e)))
        })?;
        let (_, cert) = parse_x509_certificate(&cert_bytes)
            .map_err(|e| Errors::parse("Unable to parse x509 cert", Some(Box::new(e))))?;

        let vc_type = VcType::from_str(&req_model.vc_type)?;

        let VcType::LegalRegistrationNumber(data) = vc_type else {
            return Err(Errors::unauthorized(
                format!("Cannot issue vc type: {}", vc_type),
                None
            ));
        };

        let oid_attr = cert
            .subject
            .iter_attributes()
            .find(|attr| attr.attr_type().to_id_string() == "2.5.4.97")
            .ok_or_else(|| {
                Errors::format(
                    BadFormat::Received,
                    "No organizational identifier found in certificate",
                    None
                )
            })?;

        let org_id_str = oid_attr.attr_value().as_str().map_err(|_| {
            Errors::format(BadFormat::Received, "Unable to parse organization identifier", None)
        })?;

        let prefix = match data {
            LegalRegistrationNumberTypes::TaxId => "TAX",
            LegalRegistrationNumberTypes::Euid => "EUID",
            LegalRegistrationNumberTypes::Eori => "EORI",
            LegalRegistrationNumberTypes::VatId => "VAT",
            LegalRegistrationNumberTypes::LeiCode => "LEI"
        };

        let shitty_code = org_id_str
            .split('+')
            .find(|part| part.starts_with(prefix))
            .ok_or_else(|| {
                Errors::format(
                    BadFormat::Received,
                    format!("No matching code found in cert for {:?}", data),
                    None
                )
            })?
            .to_string();

        parse_to_string(&VCData { shitty_code })
    }
}
