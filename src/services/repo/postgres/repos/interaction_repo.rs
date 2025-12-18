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

use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use super::super::super::subtraits::{BasicRepoTrait, InteractionRepoTrait};
use crate::data::entities::interaction::{Column, Entity, Model, NewModel};

#[derive(Clone)]
pub struct InteractionRepo {
    db_connection: DatabaseConnection
}

impl InteractionRepo {
    pub fn new(db_connection: DatabaseConnection) -> Self { Self { db_connection } }
}

#[async_trait]
impl BasicRepoTrait<Entity, NewModel> for InteractionRepo {
    fn db(&self) -> &DatabaseConnection { &self.db_connection }
}

#[async_trait]
impl InteractionRepoTrait for InteractionRepo {
    async fn get_by_reference(&self, reference: &str) -> anyhow::Result<Model> {
        let to_find = Entity::find().filter(Column::InteractRef.eq(reference));
        self.help_find(to_find, "reference", reference).await
    }

    async fn get_by_cont_id(&self, cont_id: &str) -> anyhow::Result<Model> {
        let to_find = Entity::find().filter(Column::ContinueId.eq(cont_id));
        self.help_find(to_find, "cont_id", cont_id).await
    }
}
