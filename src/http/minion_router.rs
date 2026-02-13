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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::Arc;

use crate::core::traits::CoreMinionTrait;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use ymir::errors::CustomToResponse;

pub struct MinionRouter {
    gru: Arc<dyn CoreMinionTrait>,
}

impl MinionRouter {
    pub fn new(gru: Arc<dyn CoreMinionTrait>) -> MinionRouter {
        MinionRouter { gru }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/all", get(Self::get_all))
            .route("/{id}", get(Self::get_by_id))
            .route("/myself", get(Self::get_me))
            .with_state(self.gru)
    }

    async fn get_all(State(gru): State<Arc<dyn CoreMinionTrait>>) -> impl IntoResponse {
        match gru.get_all().await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn get_by_id(
        State(gru): State<Arc<dyn CoreMinionTrait>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        match gru.get_by_id(id).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn get_me(State(gru): State<Arc<dyn CoreMinionTrait>>) -> impl IntoResponse {
        match gru.get_me().await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }
}
