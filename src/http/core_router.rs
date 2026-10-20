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

use crate::http::fed_catalog_router::FedCatalogRouter;
use crate::http::routes::{approver, console, docs, gate, issuer, minions, react, session, wallet};
use crate::http::{
    ApproverRouter, GateKeeperRouter, IssuerRouter, ParticipantRouter, ReactRouter,
    SessionHandlers, VerifierRouter,
};
use crate::modules::OrchestratorTrait;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::from_fn_with_state;
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{error, info, Level};
use uuid::Uuid;
use ymir::http::routes::verifier;
use ymir::http::{HealthRouter, OauthHttpMiddleware, OpenapiRouter, WalletRouter};
use ymir::services::token_validator::OauthTokenValidatorTrait;

/// Every route of the authority: protocol, console API, well-known documents, health, API docs
/// and the console's static build; panics on a bad OpenAPI path or OAuth config.
pub struct CoreRouter {
    core: Arc<dyn OrchestratorTrait>,
    openapi: String,
    validator: Arc<dyn OauthTokenValidatorTrait>,
}

impl CoreRouter {
    pub fn new(core: Arc<dyn OrchestratorTrait>) -> Self {
        let openapi = core.config().get_openapi().expect("Invalid openapi path");
        let config = core.config();
        let validator = config
            .oauth()
            .validator(config.connection().is_prod)
            .expect("Invalid oauth config");
        Self {
            core,
            openapi,
            validator,
        }
    }

    /// Every route grouped by who calls it: external (protocol routes for agents and wallets,
    /// open) and internal (console routes, all behind the OAuth guard). The `/.well-known`
    /// documents, health and the console stay at the root of the host; the rest goes under the
    /// API version.
    pub fn router(self) -> Router {
        // ===== SUB-ROUTERS =======================================================================
        let wallet_r = WalletRouter::new(self.core.clone());
        let issuer_r = IssuerRouter::new(self.core.clone());
        let gatekeeper_r = GateKeeperRouter::new(self.core.clone());
        let verifier_r = VerifierRouter::new(self.core.clone());
        let approver_r = ApproverRouter::new(self.core.clone());
        let fed_catalog_r = FedCatalogRouter::new(self.core.clone());
        let minion_r = ParticipantRouter::new(self.core.clone());
        let react_r = ReactRouter::new(self.core.clone());
        let openapi_r = OpenapiRouter::new(self.openapi.clone());
        let health_r = HealthRouter::new();

        let api_version = self.core.config().get_api_version();

        // ===== LAYERS ============================================================================
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([
                axum::http::Method::GET,
                axum::http::Method::POST,
                axum::http::Method::DELETE,
            ])
            .allow_headers(Any)
            .allow_credentials(false);

        // Console routes need an OAuth token; protocol routes are authenticated by GNAP/OID4VP.
        let guard = from_fn_with_state(self.validator.clone(), OauthHttpMiddleware::run);

        // ===== WELL-KNOWN ========================================================================
        let well_known = Router::new()
            .merge(wallet_r.well_known())
            .merge(issuer_r.well_known())
            .merge(fed_catalog_r.well_known());

        // ===== EXTERNAL: agents and wallets, authenticated by GNAP/OID4VP ========================
        let external = Router::new()
            .nest(gate::PREFIX, gatekeeper_r.external())
            .nest(issuer::PREFIX, issuer_r.external())
            .nest(verifier::PREFIX, verifier_r.external());

        // ===== INTERNAL: console users, behind the OAuth guard ===================================
        let internal = Router::new()
            .route(session::ME, get(SessionHandlers::me))
            .nest(wallet::PREFIX, wallet_r.internal())
            .nest(minions::PREFIX, minion_r.internal())
            .nest(approver::PREFIX, approver_r.internal())
            .nest(react::PREFIX, react_r.internal())
            .route_layer(guard);

        // ===== OPEN: health and API docs =========================================================
        let open = Router::new()
            .merge(health_r.router())
            .nest(docs::PREFIX, openapi_r.router());

        // ===== ROOT: health and the console ============
        let root = Router::new()
            .merge(HealthRouter::new().router())
            .route(console::ROOT, get(Self::console_root))
            .nest_service(
                console::PREFIX,
                ServeDir::new("./react/dist").fallback(ServeFile::new("./react/dist/index.html")),
            );

        // ===== ALL TOGETHER: well-known and root at the host's root, the rest under the API version
        let api = Router::new().merge(external).merge(internal).merge(open);

        Router::new()
            .merge(well_known)
            .merge(root)
            .nest(&api_version, api)
            .fallback(Self::fallback)
            .layer(cors)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(
                        |_req: &Request<_>| tracing::info_span!("request", id = %Uuid::new_v4()),
                    )
                    .on_request(|req: &Request<_>, _span: &tracing::Span| {
                        info!("{} {}", req.method(), req.uri().path());
                    })
                    .on_response(DefaultOnResponse::new().level(Level::TRACE)),
            )
    }

    async fn console_root() -> Redirect {
        Redirect::temporary(&format!("{}/", console::PREFIX))
    }

    async fn fallback() -> impl IntoResponse {
        error!("Wrong route");
        StatusCode::NOT_FOUND.into_response()
    }
}
