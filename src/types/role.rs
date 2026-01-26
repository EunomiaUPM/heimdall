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

use anyhow::bail;
use serde::{Deserialize, Serialize};
use tracing::error;
use ymir::errors::{ErrorLogTrait, Errors};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AuthorityRole {
    LegalAuthority,
    ClearingHouse,
    ClearingHouseProxy,
    DataSpaceAuthority
}

impl FromStr for AuthorityRole {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LegalAuthority" => Ok(Self::LegalAuthority),
            "ClearingHouse" => Ok(Self::ClearingHouse),
            "ClearingHouseProxy" => Ok(Self::ClearingHouseProxy),
            "DataSpaceAuthority" => Ok(Self::DataSpaceAuthority),
            "DataspaceAuthority" => Ok(Self::DataSpaceAuthority),
            _ => {
                let error = Errors::parse_new("Invalid Authority role");
                error!("{}", error.log());
                bail!(error)
            }
        }
    }
}

impl fmt::Display for AuthorityRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            AuthorityRole::LegalAuthority => "LegalAuthority",
            AuthorityRole::ClearingHouse => "ClearingHouse",
            AuthorityRole::ClearingHouseProxy => "ClearingHouseProxy",
            AuthorityRole::DataSpaceAuthority => "DataSpaceAuthority"
        };

        write!(f, "{s}")
    }
}
