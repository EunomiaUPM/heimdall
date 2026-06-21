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

use crate::modules::GatekeeperModule;
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use ymir::errors::AppResult;
use ymir::types::gnap::grant_response::GrantResponse;

pub struct GateKeeperRouter {
    gatekeeper: Arc<dyn GatekeeperModule>,
}

impl GateKeeperRouter {
    pub fn new(gatekeeper: Arc<dyn GatekeeperModule>) -> Self {
        Self { gatekeeper }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/access", post(Self::access_req))
            .route("/continue/{id}", post(Self::continue_req))
            .with_state(self.gatekeeper)
    }

    async fn access_req(
        State(gatekeeper): State<Arc<dyn GatekeeperModule>>,
        headers: HeaderMap,
        payload: Bytes,
    ) -> AppResult {
        let response = gatekeeper.manage_grant_req(payload, headers).await;

        let status = match &response {
            GrantResponse::Error(_) => StatusCode::BAD_REQUEST,
            GrantResponse::Processing { .. } => StatusCode::ACCEPTED,
            _ => StatusCode::OK,
        };

        Ok((status, Json(response)).into_response())
    }

    async fn continue_req(
        State(gatekeeper): State<Arc<dyn GatekeeperModule>>,
        headers: HeaderMap,
        Path(id): Path<String>,
        payload: Bytes,
    ) -> AppResult {
        let response = gatekeeper.manage_cont_req(id, payload, headers).await;

        let status = match &response {
            GrantResponse::Error(_) => StatusCode::BAD_REQUEST,
            GrantResponse::Processing { .. } => StatusCode::ACCEPTED,
            _ => StatusCode::OK,
        };

        Ok((status, Json(response)).into_response())
    }
}
