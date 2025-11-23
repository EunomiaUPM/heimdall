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

use super::super::super::subtraits::{BasicRepoTrait, InteractionRepoTrait};
use crate::data::entities::interaction::{Column, Entity, Model, NewModel};
use crate::errors::{ErrorLogTrait, Errors};
use anyhow::bail;
use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error;

#[derive(Clone)]
pub struct InteractionRepo {
    db_connection: DatabaseConnection,
}

impl InteractionRepo {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait]
impl BasicRepoTrait<Entity, NewModel> for InteractionRepo {
    fn db(&self) -> &DatabaseConnection {
        &self.db_connection
    }
}

#[async_trait]
impl InteractionRepoTrait for InteractionRepo {
    async fn get_by_reference(&self, reference: &str) -> anyhow::Result<Model> {
        let model = match Entity::find()
            .filter(Column::InteractRef.eq(reference))
            .one(self.db())
            .await
        {
            Ok(Some(data)) => data,
            Ok(None) => {
                let error = Errors::missing_resource_new(
                    reference,
                    &format!("Missing resource with reference: {}", reference),
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

    async fn get_by_cont_id(&self, cont_id: &str) -> anyhow::Result<Model> {
        let model = match Entity::find()
            .filter(Column::ContinueId.eq(cont_id))
            .one(self.db())
            .await
        {
            Ok(Some(data)) => data,
            Ok(None) => {
                let error = Errors::missing_resource_new(
                    cont_id,
                    &format!("Missing resource with cont_id: {}", cont_id),
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
