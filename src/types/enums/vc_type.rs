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
use crate::errors::{ErrorLogTrait, Errors};
use crate::types::enums::errors::BadFormat;
use crate::types::vcs::legal_authority::LegalRegistrationNumberTypes;
use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use tracing::error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VcType {
    LegalRegistrationNumber(LegalRegistrationNumberTypes),
    Unknown,
}

impl FromStr for VcType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LegalRegistrationNumber-tax_id" => Ok(VcType::LegalRegistrationNumber(
                LegalRegistrationNumberTypes::TaxId,
            )),
            "LegalRegistrationNumber-euid" => Ok(VcType::LegalRegistrationNumber(
                LegalRegistrationNumberTypes::Euid,
            )),
            "LegalRegistrationNumber-eori" => Ok(VcType::LegalRegistrationNumber(
                LegalRegistrationNumberTypes::Eori,
            )),
            "LegalRegistrationNumber-vat_id" => Ok(VcType::LegalRegistrationNumber(
                LegalRegistrationNumberTypes::VatId,
            )),
            "LegalRegistrationNumber-lei_code" => Ok(VcType::LegalRegistrationNumber(
                LegalRegistrationNumberTypes::LeiCode,
            )),
            _ => {
                let error = Errors::format_new(
                    BadFormat::Received,
                    &format!("Unknown credential format: {}", s),
                );
                error!("{}", error.log());
                bail!(error)
            }
        }
    }
}

impl fmt::Display for VcType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            VcType::LegalRegistrationNumber(data) => match data {
                LegalRegistrationNumberTypes::TaxId => "LegalRegistrationNumber-tax_id".to_string(),
                LegalRegistrationNumberTypes::Euid => "LegalRegistrationNumber-euid".to_string(),
                LegalRegistrationNumberTypes::Eori => "LegalRegistrationNumber-eori".to_string(),
                LegalRegistrationNumberTypes::VatId => "LegalRegistrationNumber-vat_id".to_string(),
                LegalRegistrationNumberTypes::LeiCode => {
                    "LegalRegistrationNumber-lei_code".to_string()
                }
            },
            _ => "Unknown".to_string(),
        };

        write!(f, "{s}")
    }
}

impl VcType {
    pub fn to_conf(&self) -> String {
        match self {
            VcType::LegalRegistrationNumber(_) => "LegalRegistrationNumber_jwt_vc_json".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    pub fn variants() -> Vec<VcType> {
        vec![
            VcType::Unknown,
            VcType::LegalRegistrationNumber(LegalRegistrationNumberTypes::TaxId),
            // TODO ADD MORE
        ]
    }
    pub fn name(&self) -> String {
        match self {
            VcType::LegalRegistrationNumber(_) => "LegalRegistrationNumber".to_string(),
            VcType::Unknown => "Unknown".to_string(),
        }
    }
}
