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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use ymir::data::entities::shared::participant::Model;
use ymir::errors::AppResult;

use crate::modules::ParticipantModule;

pub struct ParticipantRouter {
    participant: Arc<dyn ParticipantModule>,
}

impl ParticipantRouter {
    pub fn new(participant: Arc<dyn ParticipantModule>) -> ParticipantRouter {
        ParticipantRouter { participant }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/all", get(Self::get_all))
            .route("/{id}", get(Self::get_by_id))
            .route("/myself", get(Self::get_me))
            .with_state(self.participant)
    }

    async fn get_all(State(participant): State<Arc<dyn ParticipantModule>>) -> AppResult<Json<Vec<Model>>> {
        Ok(Json(participant.get_all().await?))
    }

    async fn get_by_id(
        State(participant): State<Arc<dyn ParticipantModule>>,
        Path(id): Path<String>,
    ) -> AppResult<Json<Model>> {
        Ok(Json(participant.get_by_id(id).await?))
    }

    async fn get_me(State(participant): State<Arc<dyn ParticipantModule>>) -> AppResult<Json<Model>> {
        Ok(Json(participant.get_me().await?))
    }
}
