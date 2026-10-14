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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use axum::Json;
use serde::Serialize;
use ymir::types::oauth::{RoleTrait, UserInfo};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeDto {
    pub user_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: String,
    pub root: bool,
}

pub struct SessionHandlers;

impl SessionHandlers {
    pub async fn me(user: UserInfo) -> Json<MeDto> {
        Json(MeDto {
            user_id: user.id().to_string(),
            username: user.username().map(str::to_string),
            email: user.email().map(str::to_string),
            role: user.role().to_string(),
            root: user.is_root(),
        })
    }
}
