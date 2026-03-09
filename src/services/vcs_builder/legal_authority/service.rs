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
use ymir::types::vcs::vc_specs::legal_reg_number::{
    LeiCode, LocalRegistrationNumber, TaxId, VatId,
};
use ymir::types::vcs::VcType;
use ymir::utils::{get_from_opt, parse_from_str, parse_to_string, parse_to_value};

use super::super::VcBuilderTrait;
use crate::config::role::{AuthorityRole, RoleConfigTrait};
use crate::services::vcs_builder::legal_authority::config::LegalAuthorityConfig;

pub struct LegalAuthorityVcBuilder {
    config: LegalAuthorityConfig,
}

impl LegalAuthorityVcBuilder {
    pub fn new(config: LegalAuthorityConfig) -> Self {
        Self { config }
    }
}

impl RoleConfigTrait for LegalAuthorityVcBuilder {
    fn get_role(&self) -> &AuthorityRole {
        &self.config.get_role()
    }
}

impl VcBuilderTrait for LegalAuthorityVcBuilder {
    fn build_vc(&self, model: &issuing::Model) -> Outcome<Value> {
        let vc_type = self.validate(&model.vc_type)?;
        info!("Building {} credential", vc_type);

        let holder_did = get_from_opt(model.holder_did.as_ref(), "holder did")?;
        let vc_data = &get_from_opt(model.credential_data.as_ref(), "credential data")?;

        let credential_subject = match vc_type {
            VcType::LeiCode => {
                let mut data = parse_from_str::<LeiCode>(vc_data)?;
                data.id = holder_did;
                parse_to_value(&data)?
            }
            VcType::LocalRegistrationNumber => {
                let mut data = parse_from_str::<LocalRegistrationNumber>(vc_data)?;
                data.id = holder_did;
                parse_to_value(&data)?
            }
            VcType::TaxId => {
                let mut data = parse_from_str::<TaxId>(vc_data)?;
                data.id = holder_did;
                parse_to_value(&data)?
            }
            VcType::VatId => {
                let mut data = parse_from_str::<VatId>(vc_data)?;
                data.id = holder_did;
                parse_to_value(&data)?
            }
            _ => unreachable!(),
        };

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

        let vc_type = self.validate(&req_model.vc_type)?;

        let cert_country = cert
            .subject
            .iter_attributes()
            .find(|attr| attr.attr_type().to_id_string() == "2.5.4.6")
            .and_then(|attr| attr.attr_value().as_str().ok())
            .map(|s| s.to_string());

        let oid_attr = cert
            .subject
            .iter_attributes()
            .find(|attr| attr.attr_type().to_id_string() == "2.5.4.97")
            .ok_or_else(|| {
                Errors::format(
                    BadFormat::Received,
                    "No organizational identifier found in certificate",
                    None,
                )
            })?;

        let org_id_str = oid_attr.attr_value().as_str().map_err(|_| {
            Errors::format(BadFormat::Received, "Unable to parse organization identifier", None)
        })?;

        let prefix = match vc_type {
            VcType::LeiCode => "LEI",
            VcType::LocalRegistrationNumber | VcType::TaxId => "NTR",
            VcType::VatId => "VAT",
            _ => unreachable!(),
        };

        let shitty_code = org_id_str
            .split('+')
            .find(|part| part.starts_with(prefix))
            .ok_or_else(|| {
                Errors::format(
                    BadFormat::Received,
                    format!("No matching code found in cert for {:?}", prefix),
                    None,
                )
            })?
            .to_string();

        match vc_type {
            VcType::LeiCode => {
                let data = LeiCode {
                    id: "".to_string(),
                    lei_code: shitty_code,
                    subdivision_country_code: None,
                    country_code: cert_country.ok_or_else(|| {
                        Errors::format(BadFormat::Received, "No country code", None)
                    })?,
                };
                parse_to_string(&data)
            }

            VcType::LocalRegistrationNumber => {
                let data = LocalRegistrationNumber { id: "".to_string(), local: shitty_code };
                parse_to_string(&data)
            }

            VcType::TaxId => {
                let data = TaxId { id: "".to_string(), tax_id: shitty_code };
                parse_to_string(&data)
            }
            VcType::VatId => {
                let data =
                    VatId { id: "".to_string(), vat_id: shitty_code, country_code: cert_country };
                parse_to_string(&data)
            }
            _ => unreachable!(),
        }
    }

    fn validate(&self, vc_type: &str) -> Outcome<VcType> {
        let vc_type = VcType::from_str(vc_type)?;

        match &vc_type {
            VcType::Eori => Err(Errors::not_impl("EORI is not impl yet", None)),
            VcType::Euid => Err(Errors::not_impl("EUID is not impl yet", None)),
            VcType::LeiCode | VcType::LocalRegistrationNumber | VcType::TaxId | VcType::VatId => {
                Ok(vc_type)
            }
            vc_type => Err(Errors::unauthorized(
                format!("Unauthorized to issue vc_type {}", vc_type.to_string()),
                None,
            )),
        }
    }
}
