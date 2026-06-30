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
use crate::services::vcs_builder::legal_authority::config::LegalAuthorityConfig;
use crate::types::need_field_for_vc;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use x509_parser::parse_x509_certificate;
use x509_parser::prelude::{AttributeTypeAndValue, X509Certificate};
use ymir::data::entities::shared::issuance;
use ymir::errors::{BadFormat, Errors, Outcome};
use ymir::types::jwt::VCJwtClaims;
use ymir::types::vcs::vc_specs::legal_reg_number::{
    LeiCode, LocalRegistrationNumber, TaxId, VatId,
};
use ymir::types::vcs::{VcType, VcTypeConfig};

const COUNTRY_OID: &str = "2.5.4.6";
const ORG_ID_OID: &str = "2.5.4.97";

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
    fn build_vc(
        &self,
        issuance: &issuance::Model,
        vc_config: VcTypeConfig,
    ) -> Outcome<VCJwtClaims> {
        let holder_did = need_field_for_vc(issuance.build_ctx.holder_did.as_deref())?;

        let role = self.config.get_role();
        if !role.available_credentials().contains(vc_config.vc_type()) {
            return Err(Errors::forbidden(
                format!("As a {} we cannot issue {}", role, vc_config),
                None,
            ));
        }

        let (code, country) = self.get_gaia_x_code(issuance, vc_config.vc_type())?;
        let credential_subject = match vc_config.vc_type() {
            VcType::LeiCode => {
                let v = LeiCode {
                    id: holder_did.to_string(),
                    lei_code: code,
                    subdivision_country_code: None,
                    country_code: country,
                };
                serde_json::to_value(&v)?
            }
            VcType::LocalRegistrationNumber => {
                let v = LocalRegistrationNumber {
                    id: holder_did.to_string(),
                    local: code,
                };
                serde_json::to_value(&v)?
            }
            VcType::TaxId => {
                let v = TaxId {
                    id: holder_did.to_string(),
                    tax_id: code,
                };
                serde_json::to_value(&v)?
            }
            VcType::VatId => {
                let v = VatId {
                    id: holder_did.to_string(),
                    vat_id: code,
                    country_code: Some(country),
                };
                serde_json::to_value(&v)?
            }
            VcType::Eori => return Err(Errors::not_impl("EORI is not impl yet", None)),
            VcType::Euid => return Err(Errors::not_impl("EUID is not impl yet", None)),
            _ => {
                return Err(Errors::forbidden(
                    format!("As a {} we cannot issue {}", role, vc_config),
                    None,
                ));
            }
        };

        self.just_build(issuance, credential_subject, vc_config)
    }
}

impl LegalAuthorityVcBuilder {
    /// Returns the Gaia-X-formatted identifier for the requested VC type, extracted
    /// from the X.509 certificate stored in the issuance context.
    ///
    /// Output format per VC type:
    /// - `LeiCode`: the 20-char ISO 17442 LEI (no prefix, no country).
    /// - `VatId`:   `<country><number>` e.g. `FR12345678901`.
    /// - `TaxId`:   `<country><number>` (accepts TIN or VAT in the cert).
    /// - `LocalRegistrationNumber`: `<country><number>`.
    fn get_gaia_x_code(
        &self,
        issuance: &issuance::Model,
        vc_type: &VcType,
    ) -> Outcome<(String, String)> {
        let cert_b64 = need_field_for_vc(issuance.build_ctx.cert.as_ref())?;
        let cert_bytes = STANDARD.decode(cert_b64).map_err(|e| {
            Errors::format(
                BadFormat::Received,
                "Unable to decode certificate (expected base64)",
                Some(Box::new(e)),
            )
        })?;
        let (_, cert) = parse_x509_certificate(&cert_bytes)
            .map_err(|e| Errors::parse("Unable to parse X.509 cert", Some(Box::new(e))))?;

        let cert_country = Self::find_subject_attr(&cert, COUNTRY_OID)?;
        let org_id_raw = Self::find_subject_attr(&cert, ORG_ID_OID)?;

        let allowed_prefixes: &[&str] = match vc_type {
            VcType::LeiCode => &["LEI"],
            VcType::VatId => &["VAT"],
            VcType::TaxId => &["TIN", "VAT"],
            VcType::LocalRegistrationNumber => &["NTR"],
            _ => {
                return Err(Errors::not_impl(
                    format!("VC type {vc_type} cannot be derived from certificate"),
                    None,
                ));
            }
        };

        let etsi_part = org_id_raw
            .split('+')
            .find(|part| {
                allowed_prefixes.iter().any(|p| {
                    part.len() >= 6 && part.starts_with(p) && part.chars().nth(5) == Some('-')
                })
            })
            .ok_or_else(|| {
                Errors::format(
                    BadFormat::Received,
                    format!(
                        "Certificate organizationIdentifier does not contain a {vc_type} \
                     value (expected one of prefixes {allowed_prefixes:?})"
                    ),
                    None,
                )
            })?;

        let prefix_country = &etsi_part[3..5];
        if prefix_country != cert_country {
            return Err(Errors::format(
                BadFormat::Received,
                format!(
                    "Country mismatch: cert country '{cert_country}' vs prefix country '{prefix_country}'"
                ),
                None,
            ));
        }

        let identifier = etsi_part.split_once('-').map(|(_, id)| id).unwrap_or("");

        let code = match vc_type {
            VcType::LeiCode => identifier.to_string(),
            _ => format!("{prefix_country}{identifier}"),
        };

        Ok((code, cert_country))
    }

    fn find_subject_attr(cert: &X509Certificate, oid: &str) -> Outcome<String> {
        cert.subject
            .iter_attributes()
            .find(|attr| attr.attr_type().to_id_string() == oid)
            .and_then(Self::attr_value_as_string)
            .ok_or_else(|| {
                Errors::format(
                    BadFormat::Received,
                    format!("Certificate missing required subject attribute {oid}"),
                    None,
                )
            })
    }

    fn attr_value_as_string(av: &AttributeTypeAndValue) -> Option<String> {
        let any = av.attr_value();
        if let Ok(s) = any.as_str() {
            return Some(s.to_string());
        }
        Some(String::from_utf8_lossy(any.data).to_string())
    }
}
