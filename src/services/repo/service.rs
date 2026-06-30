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

use crate::services::repo::RepoTrait;
use sea_orm::DatabaseConnection;
use ymir::services::repo::postgres::received::{
    RecvGrantPostgresRepo, RecvInteractionPostgresRepo, RecvVerificationPostgresRepo,
};
use ymir::services::repo::postgres::shared::{IssuancePostgresRepo, ParticipantPostgresRepo};
use ymir::services::repo::traits::received::{
    RecvGrantRepoTrait, RecvInteractionRepoTrait, RecvVerificationRepoTrait,
};
use ymir::services::repo::traits::shared::{IssuanceRepoTrait, ParticipantRepoTrait};

#[derive(Clone)]
pub struct RepoForSql {
    recv_grant_repo: Arc<dyn RecvGrantRepoTrait>,
    recv_interaction_repo: Arc<dyn RecvInteractionRepoTrait>,
    recv_verification_repo: Arc<dyn RecvVerificationRepoTrait>,
    issuance_repo: Arc<dyn IssuanceRepoTrait>,
    participant_repo: Arc<dyn ParticipantRepoTrait>,
}

impl RepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self {
            recv_grant_repo: Arc::new(RecvGrantPostgresRepo::new(db_connection.clone())),
            recv_interaction_repo: Arc::new(RecvInteractionPostgresRepo::new(
                db_connection.clone(),
            )),
            recv_verification_repo: Arc::new(RecvVerificationPostgresRepo::new(
                db_connection.clone(),
            )),
            issuance_repo: Arc::new(IssuancePostgresRepo::new(db_connection.clone())),
            participant_repo: Arc::new(ParticipantPostgresRepo::new(db_connection.clone())),
        }
    }
}

impl RepoTrait for RepoForSql {
    fn recv_grant(&self) -> Arc<dyn RecvGrantRepoTrait> {
        self.recv_grant_repo.clone()
    }

    fn recv_interaction(&self) -> Arc<dyn RecvInteractionRepoTrait> {
        self.recv_interaction_repo.clone()
    }

    fn recv_verification(&self) -> Arc<dyn RecvVerificationRepoTrait> {
        self.recv_verification_repo.clone()
    }

    fn participant(&self) -> Arc<dyn ParticipantRepoTrait> {
        self.participant_repo.clone()
    }

    fn issuance(&self) -> Arc<dyn IssuanceRepoTrait> {
        self.issuance_repo.clone()
    }
}
