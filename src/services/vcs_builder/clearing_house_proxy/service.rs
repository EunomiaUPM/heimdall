/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use super::super::VcBuilderTrait;
use crate::data::entities::{issuing, request};
use crate::errors::{ErrorLogTrait, Errors};
use crate::services::vcs_builder::legal_authority::config::{
    LegalAuthorityConfig, LegalAuthorityConfigTrait,
};
use crate::types::enums::data_model::W3cDataModelVersion;
use crate::types::enums::errors::BadFormat;
use crate::types::enums::vc_type::VcType;
use crate::types::vcs::legal_authority::{
    LegalRegistrationNumberCredSubj, LegalRegistrationNumberTypes, VCData,
};
use crate::types::vcs::{VCClaimsV1, VCClaimsV2, VCFromClaimsV1, VCIssuer};
use crate::utils::get_from_opt;
use anyhow::bail;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::{Duration, Utc};
use serde_json::Value;
use std::str::FromStr;
use tracing::{error, info};
use x509_parser::parse_x509_certificate;

pub struct GaiaProxyAuthorityBuilder {}

impl GaiaProxyAuthorityBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl VcBuilderTrait for GaiaProxyAuthorityBuilder {
    fn build_vc(&self, model: &issuing::Model) -> anyhow::Result<Value> {
        let vc_type = VcType::from_str(&model.vc_type)?;
        info!("Building {} credential", vc_type.to_string());

        let vc_data: VCData =
            serde_json::from_str(&get_from_opt(&model.credential_data, "credential data")?)?;
        let holder_did = get_from_opt(&model.holder_did, "holder did")?;
        let issuer_did = get_from_opt(&model.issuer_did, "issuer did")?;

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
        let now = Utc::now();
        let vc_type = VcType::from_str(&model.vc_type)?;
        let kk = W3cDataModelVersion::V2;
        let vc = match kk {
            W3cDataModelVersion::V1 => serde_json::to_value(VCClaimsV1 {
                exp: None,
                iat: None,
                iss: None,
                sub: None,
                vc: VCFromClaimsV1 {
                    context: vec!["https://www.w3.org/ns/credentials/v1".to_string()],
                    r#type: vec!["VerifiableCredential".to_string(), vc_type.name()],
                    id: model.credential_id.clone(),
                    credential_subject,
                    issuer: VCIssuer {
                        id: issuer_did,
                        name: "RainbowAuthority".to_string(),
                    },
                    valid_from: Some(now),
                    valid_until: Some(now + Duration::days(365)),
                },
            })?,
            W3cDataModelVersion::V2 => serde_json::to_value(VCClaimsV2 {
                exp: None,
                iat: None,
                iss: None,
                sub: None,
                context: vec!["https://www.w3.org/ns/credentials/v2".to_string()],
                r#type: vec!["VerifiableCredential".to_string(), vc_type.name()],
                id: model.credential_id.clone(),
                credential_subject,
                issuer: VCIssuer {
                    id: issuer_did,
                    name: "RainbowAuthority".to_string(),
                },
                valid_from: Some(now),
                valid_until: Some(now + Duration::days(365)),
            })?,
        };

        Ok(vc)
    }

    fn gather_data(&self, req_model: &request::Model) -> anyhow::Result<String> {
        info!("Gathering data to issue vc");

        let base_cert = req_model.cert.as_ref().ok_or_else(|| {
            let error = Errors::format_new(
                BadFormat::Received,
                "There was no cert in the Grant Request",
            );
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
                            "No organizational identifier found in certificate",
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
                        LegalRegistrationNumberTypes::LeiCode => part.starts_with("LEI"),
                    })
                    .ok_or_else(|| {
                        let error = Errors::format_new(
                            BadFormat::Received,
                            &format!("No matching code found in cert for {:?}", data),
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
