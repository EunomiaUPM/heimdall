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

use std::sync::Arc;
use async_trait::async_trait;
use crate::services::repo::RepoTrait;
use ymir::data::entities::minions::Model;

#[async_trait]
pub trait CoreMinionTrait: Send + Sync + 'static {
    fn repo(&self) -> Arc<dyn RepoTrait>;
    async fn get_all(&self) -> anyhow::Result<Vec<Model>> {
        self.repo().minions().get_all(None, None).await
    }
    async fn get_by_id(&self, id: String) -> anyhow::Result<Model> {
        self.repo().minions().get_by_id(&id).await
    }
    async fn get_me(&self) -> anyhow::Result<Model> { self.repo().minions().get_me().await }
}