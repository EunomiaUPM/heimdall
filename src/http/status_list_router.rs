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

use axum::extract::{Path, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use ymir::errors::AppResult;
use ymir::http_tracing::ExtractedHeaders;
use ymir::types::oauth::UserInfo;

use crate::data::entities::vc_status::Model;
use crate::http::routes::{issued, status_lists};
use crate::modules::{IssuedVcDetails, StatusChange, StatusListModule};

pub struct StatusListRouter {
    status: Arc<dyn StatusListModule>,
}

impl StatusListRouter {
    pub fn new(status: Arc<dyn StatusListModule>) -> Self {
        Self { status }
    }

    /// The signed status lists, open for any verifier.
    pub fn external(&self) -> Router {
        Router::new()
            .route(status_lists::BY_ID, get(Self::get_list))
            .with_state(self.status.clone())
    }

    /// Issued credentials and their status; mounted behind the OAuth guard.
    pub fn internal(self) -> Router {
        Router::new()
            .route(issued::ALL, get(Self::get_all))
            .route(issued::BY_ID, get(Self::get_one))
            .route(issued::REVOKE, post(Self::revoke))
            .route(issued::SUSPEND, post(Self::suspend))
            .route(issued::REINSTATE, post(Self::reinstate))
            .with_state(self.status)
    }

    async fn get_list(
        State(status): State<Arc<dyn StatusListModule>>,
        headers: ExtractedHeaders,
        Path((purpose, list_no)): Path<(String, i32)>,
    ) -> AppResult<impl IntoResponse> {
        let jwt = status.status_list_credential(&purpose, list_no).await?;
        Ok((
            [(CONTENT_TYPE, "application/vc+jwt")],
            headers.response_headers(),
            jwt,
        ))
    }

    async fn get_all(
        State(status): State<Arc<dyn StatusListModule>>,
        user: UserInfo,
        headers: ExtractedHeaders,
    ) -> AppResult<(HeaderMap, Json<Vec<Model>>)> {
        let issued = status.get_all_issued(&user).await?;
        Ok((headers.response_headers(), Json(issued)))
    }

    async fn get_one(
        State(status): State<Arc<dyn StatusListModule>>,
        user: UserInfo,
        headers: ExtractedHeaders,
        Path(id): Path<String>,
    ) -> AppResult<(HeaderMap, Json<IssuedVcDetails>)> {
        let issued = status.get_issued(&user, id).await?;
        Ok((headers.response_headers(), Json(issued)))
    }

    async fn revoke(
        State(status): State<Arc<dyn StatusListModule>>,
        user: UserInfo,
        headers: ExtractedHeaders,
        Path(id): Path<String>,
    ) -> AppResult<(HeaderMap, Json<Model>)> {
        let issued = status.change_status(&user, id, StatusChange::Revoke).await?;
        Ok((headers.response_headers(), Json(issued)))
    }

    async fn suspend(
        State(status): State<Arc<dyn StatusListModule>>,
        user: UserInfo,
        headers: ExtractedHeaders,
        Path(id): Path<String>,
    ) -> AppResult<(HeaderMap, Json<Model>)> {
        let issued = status.change_status(&user, id, StatusChange::Suspend).await?;
        Ok((headers.response_headers(), Json(issued)))
    }

    async fn reinstate(
        State(status): State<Arc<dyn StatusListModule>>,
        user: UserInfo,
        headers: ExtractedHeaders,
        Path(id): Path<String>,
    ) -> AppResult<(HeaderMap, Json<Model>)> {
        let issued = status.change_status(&user, id, StatusChange::Reinstate).await?;
        Ok((headers.response_headers(), Json(issued)))
    }
}
