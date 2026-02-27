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

use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use ymir::errors::Errors;
use ymir::types::vcs::vc_specs::legal_authority::LegalRegistrationNumberTypes;
use ymir::types::vcs::VcType;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AuthorityRole {
    LegalAuthority,
    ClearingHouse,
    ClearingHouseProxy,
    DataSpaceAuthority,
    EcoAuthority
}

impl FromStr for AuthorityRole {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LegalAuthority" => Ok(Self::LegalAuthority),
            "ClearingHouse" => Ok(Self::ClearingHouse),
            "ClearingHouseProxy" => Ok(Self::ClearingHouseProxy),
            "DataSpaceAuthority" => Ok(Self::DataSpaceAuthority),
            "DataspaceAuthority" => Ok(Self::DataSpaceAuthority),
            "EcoAuthority" => Ok(Self::EcoAuthority),
            _ => Err(Errors::parse("Invalid Authority Role", None))
        }
    }
}

impl fmt::Display for AuthorityRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            AuthorityRole::LegalAuthority => "LegalAuthority",
            AuthorityRole::ClearingHouse => "ClearingHouse",
            AuthorityRole::ClearingHouseProxy => "ClearingHouseProxy",
            AuthorityRole::DataSpaceAuthority => "DataSpaceAuthority",
            AuthorityRole::EcoAuthority => "EcoAuthority"
        };

        write!(f, "{s}")
    }
}

impl AuthorityRole {
    pub fn credentials(&self) -> Vec<VcType> {
        match self {
            AuthorityRole::LegalAuthority => {
                vec![VcType::LegalRegistrationNumber(LegalRegistrationNumberTypes::Eori)]
            }
            AuthorityRole::ClearingHouse => {
                vec![VcType::DataspaceParticipant]
            }
            AuthorityRole::ClearingHouseProxy => {
                vec![VcType::DataspaceParticipant]
            }
            AuthorityRole::DataSpaceAuthority => {
                vec![VcType::DataspaceParticipant]
            }
            AuthorityRole::EcoAuthority => {
                vec![
                    VcType::LegalRegistrationNumber(LegalRegistrationNumberTypes::TaxId),
                    VcType::DataspaceParticipant,
                ]
            }
        }
    }
}
