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

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use ymir::errors::{Errors, Outcome};
use ymir::services::token_validator::{
    FixedUserValidator, OauthTokenValidatorTrait, ProxiedTokenValidator,
};
use ymir::types::oauth::UserInfo;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "provider", rename_all = "snake_case")]
pub enum OauthConfig {
    Keycloak,
    Static(UserInfo),
}

impl Default for OauthConfig {
    fn default() -> Self {
        Self::Static(UserInfo::system())
    }
}

impl OauthConfig {
    pub fn validator(&self, is_prod: bool) -> Outcome<Arc<dyn OauthTokenValidatorTrait>> {
        match self {
            OauthConfig::Keycloak => Ok(Arc::new(ProxiedTokenValidator)),
            OauthConfig::Static(_) if is_prod => Err(Errors::security(
                "the static identity provider cannot run in production",
                None,
            )),
            OauthConfig::Static(user) => Ok(Arc::new(FixedUserValidator::new(user.clone()))),
        }
    }
}
