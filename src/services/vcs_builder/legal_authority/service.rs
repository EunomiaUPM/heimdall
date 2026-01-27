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

use anyhow::bail;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde_json::Value;
use tracing::{error, info};
use x509_parser::parse_x509_certificate;
use ymir::data::entities::{issuing, vc_request};
use ymir::errors::{ErrorLogTrait, Errors};
use ymir::types::errors::BadFormat;
use ymir::types::vcs::vc_specs::legal_authority::{
    LegalRegistrationNumberCredSubj, LegalRegistrationNumberTypes, VCData
};
use ymir::types::vcs::VcType;
use ymir::utils::get_from_opt;

use super::super::VcBuilderTrait;
use crate::services::vcs_builder::legal_authority::config::LegalAuthorityConfig;

pub struct LegalAuthorityVcBuilder {
    config: LegalAuthorityConfig
}

impl LegalAuthorityVcBuilder {
    pub fn new(config: LegalAuthorityConfig) -> Self { Self { config } }
}

impl VcBuilderTrait for LegalAuthorityVcBuilder {
    fn build_vc(&self, model: &issuing::Model) -> anyhow::Result<Value> {
        let vc_type = VcType::from_str(&model.vc_type)?;
        info!("Building {} credential", vc_type.to_string());

        let vc_data: VCData =
            serde_json::from_str(&get_from_opt(&model.credential_data, "credential data")?)?;
        let holder_did = get_from_opt(&model.holder_did, "holder did")?;

        let mut cred_subj = LegalRegistrationNumberCredSubj::default();
        cred_subj.id = holder_did;
        match vc_type {
            VcType::LegalRegistrationNumber(data) => match data {
                LegalRegistrationNumberTypes::TaxId => {
                    cred_subj.r#type = "gx:taxID".to_string();
                    cred_subj.tax_id = Some(vc_data.shitty_code);
                }
                LegalRegistrationNumberTypes::Euid => {
                    cred_subj.r#type = "gx:EUID".to_string();
                    cred_subj.euid = Some(vc_data.shitty_code);
                }
                LegalRegistrationNumberTypes::Eori => {
                    cred_subj.r#type = "gx:EORI".to_string();
                    cred_subj.eori = Some(vc_data.shitty_code);
                }
                LegalRegistrationNumberTypes::VatId => {
                    cred_subj.r#type = "gx:vatID".to_string();
                    cred_subj.vat_id = Some(vc_data.shitty_code);
                }
                LegalRegistrationNumberTypes::LeiCode => {
                    cred_subj.r#type = "gx:leiCode".to_string();
                    cred_subj.lei_code = Some(vc_data.shitty_code);
                }
            },
            _ => {
                let error = Errors::unauthorized_new(&format!(
                    "Cannot issue vc_type: {}",
                    vc_type.to_string()
                ));
                error!("{}", error.log());
                bail!(error)
            }
        }

        let credential_subject = serde_json::to_value(&cred_subj)?;

        self.just_build(&model, credential_subject, &self.config)
    }

    fn gather_data(&self, req_model: &vc_request::Model) -> anyhow::Result<String> {
        info!("Gathering data to issue vc");

        let base_cert = req_model.cert.as_ref().ok_or_else(|| {
            let error =
                Errors::format_new(BadFormat::Received, "There was no cert in the Grant Request");
            error!("{}", error.log());
            error
        })?;

        let cert_bytes = STANDARD.decode(base_cert)?;
        let (_, cert) = parse_x509_certificate(&cert_bytes)?;

        let vc_type = VcType::from_str(req_model.vc_type.as_str())?;

        let shitty_code = match vc_type {
            VcType::LegalRegistrationNumber(data) => {
                let oid_attr = match cert
                    .subject
                    .iter_attributes()
                    .find(|attr| attr.attr_type().to_id_string() == "2.5.4.97")
                {
                    Some(data) => data,
                    None => {
                        let error = Errors::format_new(
                            BadFormat::Received,
                            "No organizational identifier found in certificate"
                        );
                        error!("{}", error.log());
                        bail!(error)
                    }
                };

                let org_id_str = oid_attr.attr_value().as_str()?;

                let code = org_id_str
                    .split('+')
                    .find(|part| match data {
                        LegalRegistrationNumberTypes::TaxId => part.starts_with("TAX"),
                        LegalRegistrationNumberTypes::Euid => part.starts_with("EUID"),
                        LegalRegistrationNumberTypes::Eori => part.starts_with("EORI"),
                        LegalRegistrationNumberTypes::VatId => part.starts_with("VAT"),
                        LegalRegistrationNumberTypes::LeiCode => part.starts_with("LEI")
                    })
                    .ok_or_else(|| {
                        let error = Errors::format_new(
                            BadFormat::Received,
                            &format!("No matching code found in cert for {:?}", data)
                        );
                        error!("{}", error.log());
                        error
                    })?;

                code.to_string()
            }
            _ => {
                let error = Errors::unauthorized_new(&format!(
                    "Unable to issue the vc credential: {}",
                    vc_type.to_string()
                ));
                error!("{}", error.log());
                bail!(error);
            }
        };

        let data = serde_json::to_string(&VCData { shitty_code })?;

        Ok(data)
    }
}
