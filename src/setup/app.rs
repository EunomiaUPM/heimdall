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

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{serve, Router};
use axum_server::tls_rustls::RustlsConfig;
use tokio::net::TcpListener;
use tracing::info;
use ymir::config::traits::{ConnectionConfigTrait, HostsConfigTrait};
use ymir::config::types::HostType;
use ymir::errors::{Errors, Outcome};
use ymir::services::vault::{VaultService, VaultTrait};
use ymir::types::secrets::StringHelper;
use ymir::utils::expect_from_env;

use crate::config::CoreApplicationConfig;
use crate::core::CoreBuilder;
use crate::http::RainbowAuthorityRouter;

pub struct AuthorityApp;

impl AuthorityApp {
    pub async fn create_router(config: &CoreApplicationConfig, vault: Arc<VaultService>) -> Router {
        let core = CoreBuilder::from_config(config.clone(), vault).await.build();

        RainbowAuthorityRouter::new(Arc::new(core)).router()
    }

    pub async fn run_basic(config: CoreApplicationConfig, vault: Arc<VaultService>) -> Outcome<()> {
        let router = Self::create_router(&config, vault).await;

        let server_message = format!(
            "Starting Authority server in {}",
            config.hosts().get_host(HostType::Http)
        );
        info!("{}", server_message);

        let listener =
            TcpListener::bind(format!("0.0.0.0:{}", config.hosts().get_tls_port(HostType::Http)))
                .await
                .map_err(|e| {
                    Errors::crazy("Error with tcp listener", Some(anyhow::Error::from(e)))
                })?;

        serve(listener, router).await.map_err(|e| {
            Errors::crazy("Error while running basic server", Some(anyhow::Error::from(e)))
        })
    }
    pub async fn run_tls(config: &CoreApplicationConfig, vault: Arc<VaultService>) -> Outcome<()> {
        let cert = expect_from_env("VAULT_APP_ROOT_CLIENT_KEY");
        let pkey = expect_from_env("VAULT_APP_CLIENT_KEY");
        let cert: StringHelper = vault.read(None, &cert).await?;
        let pkey: StringHelper = vault.read(None, &pkey).await?;

        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Unable to install cryptography provider");

        let tls_config = RustlsConfig::from_pem(
            cert.data().as_bytes().to_vec(),
            pkey.data().as_bytes().to_vec()
        )
        .await
        .map_err(|e| {
            Errors::crazy("Errors parsing certificate stuff", Some(anyhow::Error::from(e)))
        })?;

        let router = Self::create_router(config, vault).await;

        let port = config.hosts().get_tls_port(HostType::Http);
        let addr_str = format!("0.0.0.0:{}", port);
        let addr: SocketAddr = addr_str.parse().map_err(|e| {
            Errors::crazy("Errors with socker address", Some(anyhow::Error::from(e)))
        })?;
        info!("Starting Authority server with TLS in {}", addr);

        axum_server::bind_rustls(addr, tls_config)
            .serve(router.into_make_service())
            .await
            .map_err(|e| {
                Errors::crazy("Error while running basic server", Some(anyhow::Error::from(e)))
            })?;
        Ok(())
    }
    pub async fn run(config: CoreApplicationConfig, vault: Arc<VaultService>) -> Outcome<()> {
        if config.is_tls_enabled() {
            Self::run_tls(&config, vault.clone()).await
        } else {
            Self::run_basic(config, vault).await
        }
    }
}
