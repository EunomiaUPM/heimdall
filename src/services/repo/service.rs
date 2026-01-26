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

use sea_orm::DatabaseConnection;
use ymir::services::repo::postgres::repos::{
    IssuingRepo, MinionsRepo, RecvInteractionRepo, RecvVerificationRepo, VcRequestRepo
};
use ymir::services::repo::subtraits::{
    IssuingTrait, MinionsTrait, RecvInteractionTrait, RecvVerificationTrait, VcRequestTrait
};

use crate::services::repo::RepoTrait;

#[derive(Clone)]
pub struct RepoForSql {
    request_repo: Arc<dyn VcRequestTrait>,
    interaction_repo: Arc<dyn RecvInteractionTrait>,
    verification_repo: Arc<dyn RecvVerificationTrait>,
    issuing_repo: Arc<dyn IssuingTrait>,
    minions_repo: Arc<dyn MinionsTrait>
}

impl RepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self {
            request_repo: Arc::new(VcRequestRepo::new(db_connection.clone())),
            interaction_repo: Arc::new(RecvInteractionRepo::new(db_connection.clone())),
            verification_repo: Arc::new(RecvVerificationRepo::new(db_connection.clone())),
            issuing_repo: Arc::new(IssuingRepo::new(db_connection.clone())),
            minions_repo: Arc::new(MinionsRepo::new(db_connection.clone()))
        }
    }
}

impl RepoTrait for RepoForSql {
    fn request(&self) -> Arc<dyn VcRequestTrait> { self.request_repo.clone() }

    fn interaction(&self) -> Arc<dyn RecvInteractionTrait> { self.interaction_repo.clone() }

    fn verification(&self) -> Arc<dyn RecvVerificationTrait> { self.verification_repo.clone() }
    fn minions(&self) -> Arc<dyn MinionsTrait> { self.minions_repo.clone() }

    fn issuing(&self) -> Arc<dyn IssuingTrait> { self.issuing_repo.clone() }
}
