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
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::{Form, Json, Router};
use ymir::errors::AppResult;
use ymir::types::issuing::{
    AuthServerMetadata, CredentialRequest, GiveVC, IssuerMetadata, IssuingToken, TokenRequest,
    VCCredOffer, WellKnownJwks
};
use ymir::utils::{
    extract_bearer_token, extract_form_payload, extract_payload, extract_query_param
};

use crate::core::traits::CoreIssuerTrait;

pub struct IssuerRouter {
    issuer: Arc<dyn CoreIssuerTrait>
}

impl IssuerRouter {
    pub fn new(issuer: Arc<dyn CoreIssuerTrait>) -> Self { Self { issuer } }

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
        Query(params): Query<HashMap<String, String>>
    ) -> AppResult<Json<VCCredOffer>> {
        let id = extract_query_param(&params, "id")?;
        Ok(Json(issuer.get_cred_offer_data(&id).await?))
    }

    async fn get_issuer(
        State(issuer): State<Arc<dyn CoreIssuerTrait>>
    ) -> AppResult<Json<IssuerMetadata>> {
        Ok(Json(issuer.issuer_metadata()))
    }

    async fn get_oauth_server(
        State(issuer): State<Arc<dyn CoreIssuerTrait>>
    ) -> AppResult<Json<AuthServerMetadata>> {
        Ok(Json(issuer.oauth_server_metadata()))
    }

    async fn get_jwks(
        State(issuer): State<Arc<dyn CoreIssuerTrait>>
    ) -> AppResult<Json<WellKnownJwks>> {
        Ok(Json(issuer.jwks().await?))
    }

    async fn get_token(
        State(issuer): State<Arc<dyn CoreIssuerTrait>>,
        payload: Result<Form<TokenRequest>, FormRejection>
    ) -> AppResult<Json<IssuingToken>> {
        let payload = extract_form_payload(payload)?;
        Ok(Json(issuer.get_token(payload).await?))
    }

    async fn post_credential(
        State(authority): State<Arc<dyn CoreIssuerTrait>>,
        headers: HeaderMap,
        payload: Result<Json<CredentialRequest>, JsonRejection>
    ) -> AppResult<Json<GiveVC>> {
        let payload = extract_payload(payload)?;
        let token = extract_bearer_token(&headers)?;
        Ok(Json(authority.get_credential(payload, token).await?))
    }
}
