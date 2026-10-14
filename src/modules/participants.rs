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
use async_trait::async_trait;
use chrono::Utc;
use ymir::data::entities::shared::participant::Model;
use ymir::errors::Outcome;
use ymir::services::HasWallet;
use ymir::types::participants::ParticipantType;

#[async_trait]
pub trait ParticipantModule: HasRepo + HasWallet + Send + Sync + 'static {
    fn host_url(&self) -> String;

    async fn get_all(&self) -> Outcome<Vec<Model>> {
        self.repo().participant().get_all(None, None).await
    }
    async fn get_by_id(&self, id: String) -> Outcome<Model> {
        self.repo().participant().get_by_id(&id).await
    }
    async fn get_me(&self) -> Outcome<Model> {
        let lock = self.wallet().get_identity();
        let identity = lock.read().await;
        let now = Utc::now();
        Ok(Model {
            participant_id: identity.did().id().to_string(),
            participant_nick: "Myself".to_string(),
            participant_type: ParticipantType::Authority,
            base_url: self.host_url(),
            saved_at: now,
            last_interaction: now,
            extra_fields: serde_json::json!({}),
        })
    }
}
