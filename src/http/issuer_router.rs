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

use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Form, Json, Router};
use ymir::errors::Errors;
use ymir::types::errors::BadFormat;
use ymir::types::issuing::{CredentialRequest, TokenRequest};
use ymir::utils::{extract_bearer_token, match_form_payload, match_json_payload};

use crate::core::traits::CoreIssuerTrait;

pub struct IssuerRouter {
    issuer: Arc<dyn CoreIssuerTrait>,
}

impl IssuerRouter {
    pub fn new(issuer: Arc<dyn CoreIssuerTrait>) -> Self {
        Self { issuer }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/credentialOffer", get(Self::cred_offer))
            .route("/.well-known/openid-credential-issuer", get(Self::get_issuer))
            .route("/.well-known/oauth-authorization-server", get(Self::get_oauth_server))
            .route("/jwks", get(Self::get_jwks))
            .route("/token", post(Self::get_token))
            .route("/credential", post(Self::post_credential))
            .with_state(self.issuer)
    }

    pub fn well_known(&self) -> Router {
        Router::new()
            .route("/.well-known/openid-credential-issuer", get(Self::get_issuer))
            .route("/.well-known/oauth-authorization-server", get(Self::get_oauth_server))
            .with_state(self.issuer.clone())
    }

    async fn cred_offer(
        State(issuer): State<Arc<dyn CoreIssuerTrait>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let id = match params.get("id") {
            Some(hash) => hash,
            None => {
                return Errors::format(
                    BadFormat::Received,
                    "Unable to retrieve hash from callback",
                    None,
                )
                .into_response()
            }
        };

        issuer
            .get_cred_offer_data(id)
            .await
            .map(|data| (StatusCode::OK, Json(data)))
            .into_response()
    }

    async fn get_issuer(State(issuer): State<Arc<dyn CoreIssuerTrait>>) -> impl IntoResponse {
        (StatusCode::OK, Json(issuer.issuer_metadata())).into_response()
    }

    async fn get_oauth_server(State(issuer): State<Arc<dyn CoreIssuerTrait>>) -> impl IntoResponse {
        (StatusCode::OK, Json(issuer.oauth_server_metadata())).into_response()
    }

    async fn get_jwks(State(issuer): State<Arc<dyn CoreIssuerTrait>>) -> impl IntoResponse {
        issuer.jwks().await.map(|data| (StatusCode::OK, Json(data))).into_response()
    }

    async fn get_token(
        State(issuer): State<Arc<dyn CoreIssuerTrait>>,
        payload: Result<Form<TokenRequest>, FormRejection>,
    ) -> impl IntoResponse {
        let payload = match match_form_payload(payload) {
            Ok(data) => data,
            Err(res) => return res,
        };

        issuer.get_token(payload).await.map(|data| (StatusCode::OK, Json(data))).into_response()
    }

    async fn post_credential(
        State(authority): State<Arc<dyn CoreIssuerTrait>>,
        headers: HeaderMap,
        payload: Result<Json<CredentialRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        let payload = match match_json_payload(payload) {
            Ok(data) => data,
            Err(res) => return res,
        };

        let token = match extract_bearer_token(headers) {
            Some(token) => token,
            None => return Errors::unauthorized("Missing token", None).into_response(),
        };

        authority
            .get_credential(payload, token)
            .await
            .map(|data| (StatusCode::OK, Json(data)))
            .into_response()
    }
}
