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

use std::marker::PhantomData;

use axum::extract::Request;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use uuid::Uuid;
use ymir::http::{HealthRouter, OpenapiRouter, WalletRouter};
use ymir::types::present::{Missing, Present};

use crate::http::{ApproverRouter, GateKeeperRouter, IssuerRouter, MinionRouter, VerifierRouter};

pub struct RouterBuilder<GT, ISS, VER, APP, MIN, REA, OPN, HEA, API> {
    gatekeeper: Option<GateKeeperRouter>,
    issuer: Option<IssuerRouter>,
    verifier: Option<VerifierRouter>,
    approver: Option<ApproverRouter>,
    minion: Option<MinionRouter>,
    wallet: Option<WalletRouter>,
    react: Option<bool>,
    openapi: Option<OpenapiRouter>,
    health: Option<HealthRouter>,
    api_path: Option<String>,
    _marker: PhantomData<(GT, ISS, VER, APP, MIN, REA, OPN, HEA, API)>
}

impl
    RouterBuilder<Missing, Missing, Missing, Missing, Missing, Missing, Missing, Missing, Missing>
{
    pub fn new() -> Self {
        RouterBuilder {
            gatekeeper: None,
            issuer: None,
            verifier: None,
            approver: None,
            minion: None,
            wallet: None,
            react: None,
            openapi: None,
            health: None,
            api_path: None,
            _marker: PhantomData
        }
    }
}
impl<GT, ISS, VER, APP, MIN, REA, OPN, HEA, API>
    RouterBuilder<GT, ISS, VER, APP, MIN, REA, OPN, HEA, API>
{
    pub fn gatekeeper(
        self,
        gatekeeper: GateKeeperRouter
    ) -> RouterBuilder<Present, ISS, VER, APP, MIN, REA, OPN, HEA, API> {
        RouterBuilder {
            gatekeeper: Some(gatekeeper),
            issuer: self.issuer,
            verifier: self.verifier,
            approver: self.approver,
            minion: self.minion,
            wallet: self.wallet,
            react: self.react,
            openapi: self.openapi,
            health: self.health,
            api_path: self.api_path,
            _marker: PhantomData
        }
    }

    pub fn issuer(
        self,
        issuer: IssuerRouter
    ) -> RouterBuilder<GT, Present, VER, APP, MIN, REA, OPN, HEA, API> {
        RouterBuilder {
            gatekeeper: self.gatekeeper,
            issuer: Some(issuer),
            verifier: self.verifier,
            approver: self.approver,
            minion: self.minion,
            wallet: self.wallet,
            react: self.react,
            openapi: self.openapi,
            health: self.health,
            api_path: self.api_path,
            _marker: PhantomData
        }
    }

    pub fn verifier(
        self,
        verifier: VerifierRouter
    ) -> RouterBuilder<GT, ISS, Present, APP, MIN, REA, OPN, HEA, API> {
        RouterBuilder {
            gatekeeper: self.gatekeeper,
            issuer: self.issuer,
            verifier: Some(verifier),
            approver: self.approver,
            minion: self.minion,
            wallet: self.wallet,
            react: self.react,
            openapi: self.openapi,
            health: self.health,
            api_path: self.api_path,
            _marker: PhantomData
        }
    }

    pub fn approver(
        self,
        approver: ApproverRouter
    ) -> RouterBuilder<GT, ISS, VER, Present, MIN, REA, OPN, HEA, API> {
        RouterBuilder {
            gatekeeper: self.gatekeeper,
            issuer: self.issuer,
            verifier: self.verifier,
            approver: Some(approver),
            minion: self.minion,
            wallet: self.wallet,
            react: self.react,
            openapi: self.openapi,
            health: self.health,
            api_path: self.api_path,
            _marker: PhantomData
        }
    }

    pub fn minion(
        self,
        minion: MinionRouter
    ) -> RouterBuilder<GT, ISS, VER, APP, Present, REA, OPN, HEA, API> {
        RouterBuilder {
            gatekeeper: self.gatekeeper,
            issuer: self.issuer,
            verifier: self.verifier,
            approver: self.approver,
            minion: Some(minion),
            wallet: self.wallet,
            react: self.react,
            openapi: self.openapi,
            health: self.health,
            api_path: self.api_path,
            _marker: PhantomData
        }
    }

    pub fn wallet(
        self,
        wallet: Option<WalletRouter>
    ) -> RouterBuilder<GT, ISS, VER, APP, MIN, REA, OPN, HEA, API> {
        RouterBuilder {
            gatekeeper: self.gatekeeper,
            issuer: self.issuer,
            verifier: self.verifier,
            approver: self.approver,
            minion: self.minion,
            wallet,
            react: self.react,
            openapi: self.openapi,
            health: self.health,
            api_path: self.api_path,
            _marker: PhantomData
        }
    }

    pub fn react(
        self,
        react: bool
    ) -> RouterBuilder<GT, ISS, VER, APP, MIN, Present, OPN, HEA, API> {
        RouterBuilder {
            gatekeeper: self.gatekeeper,
            issuer: self.issuer,
            verifier: self.verifier,
            approver: self.approver,
            minion: self.minion,
            wallet: self.wallet,
            react: Some(react),
            openapi: self.openapi,
            health: self.health,
            api_path: self.api_path,
            _marker: PhantomData
        }
    }

    pub fn openapi(
        self,
        openapi: OpenapiRouter
    ) -> RouterBuilder<GT, ISS, VER, APP, MIN, REA, Present, HEA, API> {
        RouterBuilder {
            gatekeeper: self.gatekeeper,
            issuer: self.issuer,
            verifier: self.verifier,
            approver: self.approver,
            minion: self.minion,
            wallet: self.wallet,
            react: self.react,
            openapi: Some(openapi),
            health: self.health,
            api_path: self.api_path,
            _marker: PhantomData
        }
    }

    pub fn health(
        self,
        health: HealthRouter
    ) -> RouterBuilder<GT, ISS, VER, APP, MIN, REA, OPN, Present, API> {
        RouterBuilder {
            gatekeeper: self.gatekeeper,
            issuer: self.issuer,
            verifier: self.verifier,
            approver: self.approver,
            minion: self.minion,
            wallet: self.wallet,
            react: self.react,
            openapi: self.openapi,
            health: Some(health),
            api_path: self.api_path,
            _marker: PhantomData
        }
    }

    pub fn api_path(
        self,
        path: String
    ) -> RouterBuilder<GT, ISS, VER, APP, MIN, REA, OPN, HEA, Present> {
        RouterBuilder {
            gatekeeper: self.gatekeeper,
            issuer: self.issuer,
            verifier: self.verifier,
            approver: self.approver,
            minion: self.minion,
            wallet: self.wallet,
            react: self.react,
            openapi: self.openapi,
            health: self.health,
            api_path: Some(path),
            _marker: PhantomData
        }
    }
}

impl
    RouterBuilder<Present, Present, Present, Present, Present, Present, Present, Present, Present>
{
    pub fn build(self) -> Router {
        let issuer = self.issuer.unwrap();

        let base_router = Router::new().merge(issuer.well_known());

        let router = Router::new()
            // .merge(issuer.well_known())
            .nest("/health", self.health.unwrap().router())
            .nest("/minions", self.minion.unwrap().router())
            .nest("/approver", self.approver.unwrap().router())
            .nest("/gate", self.gatekeeper.unwrap().router())
            .nest("/issuer", issuer.router())
            .nest("/verifier", self.verifier.unwrap().router())
            .nest("/docs", self.openapi.unwrap().router());

        let (base_router, router) = if let Some(wallet) = self.wallet {
            let base_router = base_router.merge(wallet.well_known());
            let router = router.nest("/wallet", wallet.router());
            (base_router, router)
        } else {
            (base_router, router)
        };

        let base_router = if self.react.unwrap() {
            base_router.nest_service(
                "/admin",
                ServeDir::new("./react/dist")
                    .not_found_service(ServeFile::new("./react/dist/index.html"))
            )
        } else {
            base_router
        };
        let router = base_router.nest(&self.api_path.unwrap(), router);
        router
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(
                        |_req: &Request<_>| tracing::info_span!("request", id = %Uuid::new_v4())
                    )
                    .on_request(|req: &Request<_>, _span: &tracing::Span| {
                        info!("{} {}", req.method(), req.uri().path());
                    })
                    .on_response(DefaultOnResponse::new().level(Level::TRACE))
            )
            .layer(CorsLayer::permissive())
    }
}
