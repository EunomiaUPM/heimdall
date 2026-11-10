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

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// A credential heimdall issued and its position in the status lists.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "vc_status")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub credential_id: String,
    #[sea_orm(unique)]
    pub issuance_id: String,
    pub vc_type: String,
    pub holder_did: String,
    pub subject_name: String,
    pub list_no: i32,
    pub idx: i32,
    pub issued_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub suspended_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug)]
pub struct Plan {
    pub credential_id: String,
    pub issuance_id: String,
    pub vc_type: String,
    pub holder_did: String,
    pub subject_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
