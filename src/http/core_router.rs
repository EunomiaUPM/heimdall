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

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use tracing::error;
use ymir::http::{HealthRouter, OpenapiRouter, WalletRouter};

use crate::core::traits::CoreTrait;
use crate::http::builder::RouterBuilder;
use crate::http::{ApproverRouter, GateKeeperRouter, IssuerRouter, MinionRouter, VerifierRouter};

pub struct RainbowAuthorityRouter {
    core: Arc<dyn CoreTrait>,
    openapi: String
}

impl RainbowAuthorityRouter {
    pub fn new(core: Arc<dyn CoreTrait>) -> Self {
        let openapi = core.config().get_openapi().expect("Invalid openapi path");
        Self { core, openapi }
    }

    pub fn router(self) -> Router {
        let wallet = match self.core.config().is_wallet_active() {
            true => Some(WalletRouter::new(self.core.clone())),
            false => None
        };

        let router = RouterBuilder::new()
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

        // Builder already includes a layer for logging
        router.fallback(Self::fallback)
    }
    async fn fallback() -> impl IntoResponse {
        error!("Wrong route");
        StatusCode::NOT_FOUND.into_response()
    }
}
