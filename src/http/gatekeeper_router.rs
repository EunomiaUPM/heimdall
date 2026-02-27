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

use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use ymir::errors::AppResult;
use ymir::types::gnap::grant_request::GrantRequest;
use ymir::types::gnap::RefBody;
use ymir::utils::{extract_gnap_token, extract_payload};

use crate::core::traits::CoreGatekeeperTrait;

pub struct GateKeeperRouter {
    gatekeeper: Arc<dyn CoreGatekeeperTrait>
}

impl GateKeeperRouter {
    pub fn new(gatekeeper: Arc<dyn CoreGatekeeperTrait>) -> Self { Self { gatekeeper } }

    pub fn router(self) -> Router {
        Router::new()
            .route("/access", post(Self::access_req))
            .route("/continue/{id}", post(Self::continue_req))
            .with_state(self.gatekeeper)
    }

    async fn access_req(
        State(gatekeeper): State<Arc<dyn CoreGatekeeperTrait>>,
        payload: Result<Json<GrantRequest>, JsonRejection>
    ) -> AppResult {
        let payload = extract_payload(payload)?;
        Ok(gatekeeper
            .manage_req(payload)
            .await
            .map(Json)
            .map_err(|e| (StatusCode::BAD_REQUEST, Json(e)))
            .into_response())
    }

    async fn continue_req(
        State(authority): State<Arc<dyn CoreGatekeeperTrait>>,
        headers: HeaderMap,
        Path(id): Path<String>,
        payload: Result<Json<RefBody>, JsonRejection>
    ) -> AppResult<String> {
        let token = extract_gnap_token(headers)?;
        let payload = extract_payload(payload)?;
        authority.manage_cont_req(id, payload, token).await
    }
}
