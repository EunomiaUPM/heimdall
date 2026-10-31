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

use axum::extract::State;
use axum::http::HeaderMap;
use axum::routing::get;
use axum::{Json, Router};
use ymir::data::entities::shared::participant::Model;
use ymir::errors::AppResult;
use ymir::http::PathId;
use ymir::http_tracing::ExtractedHeaders;

use crate::http::routes::minions;
use crate::modules::ParticipantModule;

pub struct ParticipantRouter {
    participant: Arc<dyn ParticipantModule>,
}

impl ParticipantRouter {
    pub fn new(participant: Arc<dyn ParticipantModule>) -> ParticipantRouter {
        ParticipantRouter { participant }
    }

    /// Participant routes; mounted behind the OAuth guard.
    pub fn internal(self) -> Router {
        Router::new()
            .route(minions::ALL, get(Self::get_all))
            .route(minions::BY_ID, get(Self::get_by_id))
            .route(minions::MYSELF, get(Self::get_me))
            .with_state(self.participant)
    }

    async fn get_all(
        State(participant): State<Arc<dyn ParticipantModule>>,
        headers: ExtractedHeaders,
    ) -> AppResult<(HeaderMap, Json<Vec<Model>>)> {
        let participants = participant.get_all().await?;
        Ok((headers.response_headers(), Json(participants)))
    }

    async fn get_by_id(
        State(participant): State<Arc<dyn ParticipantModule>>,
        headers: ExtractedHeaders,
        PathId(id): PathId,
    ) -> AppResult<(HeaderMap, Json<Model>)> {
        let found = participant.get_by_id(id).await?;
        Ok((headers.response_headers(), Json(found)))
    }

    async fn get_me(
        State(participant): State<Arc<dyn ParticipantModule>>,
        headers: ExtractedHeaders,
    ) -> AppResult<(HeaderMap, Json<Model>)> {
        let me = participant.get_me().await?;
        Ok((headers.response_headers(), Json(me)))
    }
}
