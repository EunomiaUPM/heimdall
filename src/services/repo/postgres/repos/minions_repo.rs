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

use super::super::super::subtraits::{BasicRepoTrait, MinionsRepoTrait};
use crate::data::entities::minions::{Column, Entity, Model, NewModel};
use crate::data::IntoActiveSet;
use crate::errors::{ErrorLogTrait, Errors};
use anyhow::bail;
use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error;

#[derive(Clone)]
pub struct MinionsRepo {
    db_connection: DatabaseConnection,
}

impl MinionsRepo {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl BasicRepoTrait<Entity, NewModel> for MinionsRepo {
    fn db(&self) -> &DatabaseConnection {
        &self.db_connection
    }
}

#[async_trait]
impl MinionsRepoTrait for MinionsRepo {
    async fn get_me(&self) -> anyhow::Result<Model> {
        let me = match Entity::find()
            .filter(Column::IsMe.eq(true))
            .one(self.db())
            .await
        {
            Ok(Some(data)) => data,
            Ok(None) => {
                let error = Errors::missing_resource_new("Myself", "Missing resource myself");
                error!("{}", error.log());
                bail!(error)
            }
            Err(e) => {
                let error = Errors::database_new(&e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        };
        Ok(me)
    }

    async fn force_create(&self, minion: NewModel) -> anyhow::Result<Model> {
        let active_mate = minion.to_active();
        let mate = match Entity::insert(active_mate)
            .on_conflict(
                OnConflict::column(Column::ParticipantId)
                    .update_columns([
                        Column::BaseUrl,
                        Column::LastInteraction,
                        Column::VcUri,
                        Column::ParticipantSlug,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(self.db())
            .await
        {
            Ok(mate) => mate,
            Err(e) => {
                let error = Errors::database_new(&e.to_string());
                error!("{}", error.log());
                bail!(error)
            }
        };
        Ok(mate)
    }
}
