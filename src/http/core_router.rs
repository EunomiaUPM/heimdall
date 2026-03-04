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

use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{error, info, Level};
use uuid::Uuid;
use ymir::http::{HealthRouter, OpenapiRouter, WalletRouter};

use crate::core::traits::CoreTrait;
use crate::http::builder::RouterBuilder;
use crate::http::{
    ApproverRouter, GateKeeperRouter, IssuerRouter, MinionRouter, ReactRouter, VerifierRouter,
};

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
        let wallet = match self.core.config().is_wallet_active() {
            true => Some(WalletRouter::new(self.core.clone())),
            false => None,
        };

        let mut router = RouterBuilder::new()
            .gatekeeper(GateKeeperRouter::new(self.core.clone()))
            .issuer(IssuerRouter::new(self.core.clone()))
            .verifier(VerifierRouter::new(self.core.clone()))
            .approver(ApproverRouter::new(self.core.clone()))
            .minion(MinionRouter::new(self.core.clone()))
            .wallet(wallet)
            .react(self.core.config().is_react())
            .openapi(OpenapiRouter::new(self.openapi.clone()))
            .health(HealthRouter::new())
            .api_path(self.core.config().get_api_version())
            .build();

        // Manual override for react_router merging without changing builder struct
        if self.core.config().is_react() {
            let sse_router = ReactRouter::new(self.core.clone()).router();
            let mount_path = format!("{}/react", self.core.config().get_api_version());
            router = router.nest(&mount_path, sse_router);
        }

        router
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
