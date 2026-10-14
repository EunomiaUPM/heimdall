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
use crate::http::{
    ApproverRouter, GateKeeperRouter, IssuerRouter, ParticipantRouter, ReactRouter,
    SessionHandlers, VerifierRouter,
};
use crate::modules::OrchestratorTrait;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::middleware::from_fn_with_state;
use axum::routing::get;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{error, info, Level};
use uuid::Uuid;
use ymir::http::{HealthRouter, OauthHttpMiddleware, OpenapiRouter, WalletRouter};
use ymir::services::token_validator::OauthTokenValidatorTrait;

pub struct RainbowAuthorityRouter {
    core: Arc<dyn OrchestratorTrait>,
    openapi: String,
    validator: Arc<dyn OauthTokenValidatorTrait>,
}

impl RainbowAuthorityRouter {
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

    pub fn router(self) -> Router {
        let api_version = self.core.config().get_api_version();
        let wallet = WalletRouter::new(self.core.clone());
        let issuer = IssuerRouter::new(self.core.clone());
        let gatekeeper = GateKeeperRouter::new(self.core.clone());
        let verifier = VerifierRouter::new(self.core.clone());
        let approver = ApproverRouter::new(self.core.clone());
        let fed_catalog = FedCatalogRouter::new(self.core.clone());
        let minion = ParticipantRouter::new(self.core.clone());
        let health = HealthRouter::new();
        let openapi = OpenapiRouter::new(self.openapi.clone());
        let react = ReactRouter::new(self.core.clone());

        let base_router = Router::new()
            .route("/", get(Self::console_root))
            .merge(health.router())
            .merge(wallet.well_known())
            .merge(issuer.well_known())
            .merge(fed_catalog.well_known())
            .nest_service(
                "/admin",
                ServeDir::new("./react/dist")
                    .fallback(ServeFile::new("./react/dist/index.html")),
            );

        let guard = from_fn_with_state(self.validator.clone(), OauthHttpMiddleware::run);

        let user_router = Router::new()
            .route("/me", get(SessionHandlers::me))
            .nest("/wallet", wallet.internal())
            .nest("/minions", minion.router())
            .nest("/approver", approver.router())
            .nest("/react", react.router())
            .route_layer(guard);

        let api_router = Router::new()
            .merge(user_router)
            .nest("/gate", gatekeeper.router())
            .nest("/issuer", issuer.router())
            .nest("/verifier", verifier.router())
            .nest("/docs", openapi.router());

        base_router
            .nest(&api_version, api_router)
            .fallback(Self::fallback)
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
            .layer(CorsLayer::permissive())
    }

    async fn console_root() -> Redirect {
        Redirect::temporary("/admin/")
    }

    async fn fallback() -> impl IntoResponse {
        error!("Wrong route");
        StatusCode::NOT_FOUND.into_response()
    }
}
