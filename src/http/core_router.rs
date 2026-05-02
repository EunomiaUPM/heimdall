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

use crate::core::traits::CoreTrait;
use crate::http::fed_catalog_router::FedCatalogRouter;
use crate::http::{
    ApproverRouter, GateKeeperRouter, IssuerRouter, MinionRouter, ReactRouter, VerifierRouter,
};
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{error, info, Level};
use uuid::Uuid;
use ymir::config::types::HostType;
use ymir::http::{HealthRouter, OpenapiRouter, WalletRouter};
use ymir::types::dids::{DidService, DidServiceType};

pub struct RainbowAuthorityRouter {
    core: Arc<dyn CoreTrait>,
    openapi: String,
}

impl RainbowAuthorityRouter {
    pub fn new(core: Arc<dyn CoreTrait>) -> Self {
        let openapi = core.config().get_openapi().expect("Invalid openapi path");
        Self { core, openapi }
    }

    pub fn router(self) -> Router {
        let api_version = self.core.config().get_api_version();
        let issuer = IssuerRouter::new(self.core.clone());
        let gatekeeper = GateKeeperRouter::new(self.core.clone());
        let verifier = VerifierRouter::new(self.core.clone());
        let approver = ApproverRouter::new(self.core.clone());
        let fed_catalog = FedCatalogRouter::new(self.core.clone());
        let minion = MinionRouter::new(self.core.clone());
        let health = HealthRouter::new();
        let openapi = OpenapiRouter::new(self.openapi.clone());

        let mut base_router = Router::new().merge(issuer.well_known());

        let mut api_router = Router::new()
            .merge(health.router())
            .nest("/minions", minion.router())
            .nest("/approver", approver.router())
            .nest("/gate", gatekeeper.router())
            .nest("/issuer", issuer.router())
            .nest("/verifier", verifier.router())
            .nest("/docs", openapi.router());

        if self.core.config().is_wallet_active() {
            let services = vec![
                DidService::basic(
                    DidServiceType::CredentialIssuer,
                    format!(
                        "{}{}/gate/access",
                        self.core.config().get_host(HostType::Http),
                        api_version
                    ),
                ),
                DidService::basic(
                    DidServiceType::FederatedCatalog,
                    format!(
                        "{}/.well-known/federated-catalog",
                        self.core.config().get_host(HostType::Http),
                    ),
                ),
            ];
            let wallet = WalletRouter::new(self.core.clone());
            base_router = base_router.merge(wallet.well_known(Some(services)));
            api_router = api_router.nest("/wallet", wallet.router());
        }

        if self.core.config().is_react() {
            let react = ReactRouter::new(self.core.clone());
            base_router = base_router.nest_service(
                "/admin",
                ServeDir::new("./react/dist")
                    .not_found_service(ServeFile::new("./react/dist/index.html")),
            );
            api_router = api_router.nest("/react", react.router());
        }

        base_router
            .merge(fed_catalog.well_known())
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

    async fn fallback() -> impl IntoResponse {
        error!("Wrong route");
        StatusCode::NOT_FOUND.into_response()
    }
}
