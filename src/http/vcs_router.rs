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

use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::Value;
use ymir::data::entities::received::grant::Model;
use ymir::errors::AppResult;
use ymir::types::gnap::InteractionFinishResponse;
use ymir::types::gnap::VcDecisionApproval;
use ymir::utils::extract_payload;

use crate::modules::ApproverModule;

pub struct ApproverRouter {
    approver: Arc<dyn ApproverModule>,
}

impl ApproverRouter {
    pub fn new(approver: Arc<dyn ApproverModule>) -> Self {
        Self { approver }
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/all", get(Self::get_all_requests))
            .route(
                "/{id}",
                get(Self::get_one_request).post(Self::manage_request),
            )
            .route("/{id}/details", get(Self::get_one_with_details))
            .with_state(self.approver)
    }

    async fn get_all_requests(
        State(approver): State<Arc<dyn ApproverModule>>,
    ) -> AppResult<Json<Vec<Model>>> {
        Ok(Json(approver.get_all().await?))
    }

    async fn get_one_request(
        State(approver): State<Arc<dyn ApproverModule>>,
        Path(id): Path<String>,
    ) -> AppResult<Json<Model>> {
        Ok(Json(approver.get_by_id(id).await?))
    }
    async fn get_one_with_details(
        State(gatekeeper): State<Arc<dyn ApproverModule>>,
        Path(id): Path<String>,
    ) -> AppResult<Json<Value>> {
        Ok(Json(gatekeeper.get_by_id_with_details(id).await?))
    }

    async fn manage_request(
        State(gatekeeper): State<Arc<dyn ApproverModule>>,
        Path(id): Path<String>,
        payload: Result<Json<VcDecisionApproval>, JsonRejection>,
    ) -> AppResult {
        let payload = extract_payload(payload)?;
        Ok(match gatekeeper.manage_req(id, payload).await? {
            InteractionFinishResponse::Success(Some(uri)) => (StatusCode::OK, uri).into_response(),
            InteractionFinishResponse::Success(None) => StatusCode::OK.into_response(),
            InteractionFinishResponse::Failure(Some(uri)) => {
                (StatusCode::UNPROCESSABLE_ENTITY, uri).into_response()
            }
            InteractionFinishResponse::Failure(None) => {
                StatusCode::UNPROCESSABLE_ENTITY.into_response()
            }
        })
    }
}
