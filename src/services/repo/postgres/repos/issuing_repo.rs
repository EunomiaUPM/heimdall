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

use super::super::super::subtraits::{BasicRepoTrait, IssuingRepoTrait};
use crate::data::entities::issuing::{Column, Entity, Model, NewModel};
use crate::errors::{ErrorLogTrait, Errors};
use anyhow::bail;
use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error;

#[derive(Clone)]
pub struct IssuingRepo {
    db_connection: DatabaseConnection,
}

impl IssuingRepo {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl BasicRepoTrait<Entity, NewModel> for IssuingRepo {
    fn db(&self) -> &DatabaseConnection {
        &self.db_connection
    }
}

#[async_trait]
impl IssuingRepoTrait for IssuingRepo {
    async fn get_by_tx_code(&self, code: &str) -> anyhow::Result<Model> {
        let model = match Entity::find()
            .filter(Column::TxCode.eq(code))
            .one(self.db())
            .await
        {
            Ok(Some(data)) => data,
            Ok(None) => {
                let error = Errors::missing_resource_new(
                    code,
                    &format!("Missing resource with code: {}", code),
                );
                error!("{}", error.log());
                bail!(error)
            }
            Err(e) => {
                let error = Errors::database_new(&e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        };
        Ok(model)
    }

    async fn get_by_token(&self, token: &str) -> anyhow::Result<Model> {
        let model = match Entity::find()
            .filter(Column::Token.eq(token))
            .one(self.db())
            .await
        {
            Ok(Some(data)) => data,
            Ok(None) => {
                let error = Errors::missing_resource_new(
                    token,
                    &format!("Missing resource with token: {}", token),
                );
                error!("{}", error.log());
                bail!(error)
            }
            Err(e) => {
                let error = Errors::database_new(&e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        };
        Ok(model)
    }
}
