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

//! Heimdall's planes as ymir modules over the shared `Core`. Each mounts the routers it already
//! had, with its own prefix, CORS and, for console routes, the OAuth guard.

use std::sync::Arc;

use axum::extract::Request;
use axum::middleware::{from_fn, from_fn_with_state, Next};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use ymir::http::{OauthHttpMiddleware, OpenapiRouter, WalletRouter};
use ymir::module_loader::service_module::ServiceModuleTrait;
use ymir::services::token_validator::OauthTokenValidatorTrait;

use crate::core::Core;
use crate::http::routes::{approver, console, docs, gate, issuer, minions, react, session, wallet};
use crate::http::{
    ApproverRouter, FedCatalogRouter, GateKeeperRouter, IssuerRouter, ParticipantRouter,
    ReactRouter, SessionHandlers, VerifierRouter,
};
use ymir::http::routes::verifier;

/// What every plane needs: the core, the API version prefix and the console's token validator.
#[derive(Clone)]
pub struct PlaneContext {
    core: Arc<Core>,
    api_version: String,
    validator: Arc<dyn OauthTokenValidatorTrait>,
}

impl PlaneContext {
    pub fn new(
        core: Arc<Core>,
        api_version: String,
        validator: Arc<dyn OauthTokenValidatorTrait>,
    ) -> Self {
        Self {
            core,
            api_version,
            validator,
        }
    }

    /// `prefix` under the API version.
    fn api(&self, prefix: &str) -> String {
        format!("{}{}", self.api_version, prefix)
    }

    /// Console routes need an OAuth token; protocol routes are authenticated by GNAP/OID4VP.
    fn guarded(&self, router: Router) -> Router {
        router.route_layer(from_fn_with_state(
            self.validator.clone(),
            OauthHttpMiddleware::run,
        ))
    }

    /// Browsers (wallets, the console in dev) call from other origins.
    fn cors(router: Router) -> Router {
        router.layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::DELETE,
                ])
                .allow_headers(Any)
                .allow_credentials(false),
        )
    }

    /// A plane's routes, mounted at the root of the host.
    fn mount(router: Router) -> Option<(String, Router)> {
        Some((String::new(), Self::cors(router)))
    }
}

/// Wallet documents at the root, the console's wallet API behind the guard.
pub struct WalletPlane(PlaneContext);

impl WalletPlane {
    pub fn new(ctx: PlaneContext) -> Self {
        Self(ctx)
    }
}

impl ServiceModuleTrait for WalletPlane {
    fn name(&self) -> &'static str {
        "wallet"
    }

    fn http(&self) -> Option<(String, Router)> {
        let r = WalletRouter::new(self.0.core.clone());
        let router = Router::new()
            .merge(r.well_known())
            .nest(&self.0.api(wallet::PREFIX), self.0.guarded(r.internal()));
        PlaneContext::mount(router)
    }
}

/// OID4VCI issuer: metadata at the root, protocol under the API.
pub struct IssuerPlane(PlaneContext);

impl IssuerPlane {
    pub fn new(ctx: PlaneContext) -> Self {
        Self(ctx)
    }
}

impl ServiceModuleTrait for IssuerPlane {
    fn name(&self) -> &'static str {
        "issuer"
    }

    fn http(&self) -> Option<(String, Router)> {
        let r = IssuerRouter::new(self.0.core.clone());
        let router = Router::new()
            .merge(r.well_known())
            .nest(&self.0.api(issuer::PREFIX), r.external());
        PlaneContext::mount(router)
    }
}

/// OID4VP verifier protocol.
pub struct VerifierPlane(PlaneContext);

impl VerifierPlane {
    pub fn new(ctx: PlaneContext) -> Self {
        Self(ctx)
    }
}

impl ServiceModuleTrait for VerifierPlane {
    fn name(&self) -> &'static str {
        "verifier"
    }

    fn http(&self) -> Option<(String, Router)> {
        let r = VerifierRouter::new(self.0.core.clone());
        PlaneContext::mount(Router::new().nest(&self.0.api(verifier::PREFIX), r.external()))
    }
}

/// GNAP grant endpoints.
pub struct GatekeeperPlane(PlaneContext);

impl GatekeeperPlane {
    pub fn new(ctx: PlaneContext) -> Self {
        Self(ctx)
    }
}

impl ServiceModuleTrait for GatekeeperPlane {
    fn name(&self) -> &'static str {
        "gatekeeper"
    }

    fn http(&self) -> Option<(String, Router)> {
        let r = GateKeeperRouter::new(self.0.core.clone());
        PlaneContext::mount(Router::new().nest(&self.0.api(gate::PREFIX), r.external()))
    }
}

/// Console approval of credential requests.
pub struct ApproverPlane(PlaneContext);

impl ApproverPlane {
    pub fn new(ctx: PlaneContext) -> Self {
        Self(ctx)
    }
}

impl ServiceModuleTrait for ApproverPlane {
    fn name(&self) -> &'static str {
        "approver"
    }

    fn http(&self) -> Option<(String, Router)> {
        let r = ApproverRouter::new(self.0.core.clone());
        let router =
            Router::new().nest(&self.0.api(approver::PREFIX), self.0.guarded(r.internal()));
        PlaneContext::mount(router)
    }
}

/// Console view of the participants.
pub struct ParticipantsPlane(PlaneContext);

impl ParticipantsPlane {
    pub fn new(ctx: PlaneContext) -> Self {
        Self(ctx)
    }
}

impl ServiceModuleTrait for ParticipantsPlane {
    fn name(&self) -> &'static str {
        "participants"
    }

    fn http(&self) -> Option<(String, Router)> {
        let r = ParticipantRouter::new(self.0.core.clone());
        let router = Router::new().nest(&self.0.api(minions::PREFIX), self.0.guarded(r.internal()));
        PlaneContext::mount(router)
    }
}

/// Federated catalog document at the root.
pub struct FedCatalogPlane(PlaneContext);

impl FedCatalogPlane {
    pub fn new(ctx: PlaneContext) -> Self {
        Self(ctx)
    }
}

impl ServiceModuleTrait for FedCatalogPlane {
    fn name(&self) -> &'static str {
        "fed-catalog"
    }

    fn http(&self) -> Option<(String, Router)> {
        PlaneContext::mount(FedCatalogRouter::new(self.0.core.clone()).well_known())
    }
}

/// Console notification stream.
pub struct NotifierPlane(PlaneContext);

impl NotifierPlane {
    pub fn new(ctx: PlaneContext) -> Self {
        Self(ctx)
    }
}

impl ServiceModuleTrait for NotifierPlane {
    fn name(&self) -> &'static str {
        "notifier"
    }

    fn http(&self) -> Option<(String, Router)> {
        let r = ReactRouter::new(self.0.core.clone());
        let router = Router::new().nest(&self.0.api(react::PREFIX), self.0.guarded(r.internal()));
        PlaneContext::mount(router)
    }
}

/// Console session, API docs and the console's static build.
pub struct ConsolePlane {
    ctx: PlaneContext,
    openapi: String,
}

impl ConsolePlane {
    pub fn new(ctx: PlaneContext, openapi: String) -> Self {
        Self { ctx, openapi }
    }

    async fn console_root() -> Redirect {
        Redirect::temporary(&format!("{}/", console::PREFIX))
    }

    async fn console_slash(req: Request, next: Next) -> Response {
        if req.uri().path() == console::PREFIX {
            return Self::console_root().await.into_response();
        }
        next.run(req).await
    }
}

impl ServiceModuleTrait for ConsolePlane {
    fn name(&self) -> &'static str {
        "console"
    }

    fn http(&self) -> Option<(String, Router)> {
        let session = self
            .ctx
            .guarded(Router::new().route(session::ME, get(SessionHandlers::me)));
        let api = Router::new().merge(session).nest(
            docs::PREFIX,
            OpenapiRouter::new(self.openapi.clone()).router(),
        );
        let root = Router::new()
            .route(console::ROOT, get(Self::console_root))
            .nest_service(
                console::PREFIX,
                ServeDir::new("./react/dist").fallback(ServeFile::new("./react/dist/index.html")),
            )
            .layer(from_fn(Self::console_slash));
        PlaneContext::mount(root.nest(&self.ctx.api_version, api))
    }
}
