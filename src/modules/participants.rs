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

use crate::services::HasRepo;
use crate::services::repo::RepoTrait;
use async_trait::async_trait;
use ymir::data::entities::shared::participant::Model;
use ymir::errors::Outcome;

#[async_trait]
pub trait ParticipantModule: HasRepo + Send + Sync + 'static {
    async fn get_all(&self) -> Outcome<Vec<Model>> {
        self.repo().participant().get_all(None, None).await
    }
    async fn get_by_id(&self, id: String) -> Outcome<Model> {
        self.repo().participant().get_by_id(&id).await
    }
    async fn get_me(&self) -> Outcome<Model> {
        self.repo().participant().get_me().await
    }
}
