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

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::{serve, Router};
use axum_server::tls_rustls::RustlsConfig;
use tokio::net::TcpListener;
use tracing::{debug, error, info, warn};
use ymir::config::traits::{ApiConfigTrait, ConnectionConfigTrait, HostsConfigTrait};
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
        let core = CoreBuilder::from_config(config.clone(), vault)
            .await
            .build();

        RainbowAuthorityRouter::new(Arc::new(core)).router()
    }

    pub async fn run_basic(config: CoreApplicationConfig, vault: Arc<VaultService>) -> Outcome<()> {
        let router = Self::create_router(&config, vault).await;

        let port = config.get_internal_port(HostType::Http);
        let server_message = format!("Starting Authority server in {}", port);
        info!("{}", server_message);

        let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
            .await
            .map_err(|e| Errors::crazy("Error with tcp listener", Some(Box::new(e))))?;

        Self::spawn_auto_link(config.get_api_version(), port, false);

        serve(listener, router)
            .await
            .map_err(|e| Errors::crazy("Error while running basic server", Some(Box::new(e))))
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
            pkey.data().as_bytes().to_vec(),
        )
        .await
        .map_err(|e| Errors::crazy("Errors parsing certificate stuff", Some(Box::new(e))))?;

        let router = Self::create_router(config, vault).await;

        let port = config.get_internal_port(HostType::Http);
        let addr_str = format!("0.0.0.0:{}", port);
        let addr: SocketAddr = addr_str
            .parse()
            .map_err(|e| Errors::crazy("Errors with socker address", Some(Box::new(e))))?;
        info!("Starting Authority server with TLS in {}", addr);

        Self::spawn_auto_link(config.get_api_version(), port, true);

        axum_server::bind_rustls(addr, tls_config)
            .serve(router.into_make_service())
            .await
            .map_err(|e| Errors::crazy("Error while running basic server", Some(Box::new(e))))?;
        Ok(())
    }
    pub async fn run(config: CoreApplicationConfig, vault: Arc<VaultService>) -> Outcome<()> {
        if config.is_prod() && !config.has_tls_proxy() {
            Self::run_tls(&config, vault.clone()).await
        } else {
            Self::run_basic(config, vault).await
        }
    }

    fn spawn_auto_link(api_version: String, port: String, tls: bool) {
        tokio::spawn(async move {
            let scheme = if tls { "https" } else { "http" };
            let url = format!("{}://127.0.0.1:{}{}/wallet/link", scheme, port, api_version);

            let client = match reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
            {
                Ok(c) => c,
                Err(e) => {
                    error!("Auto wallet link: failed to build HTTP client: {}", e);
                    return;
                }
            };

            let max_attempts = 20;
            for attempt in 1..=max_attempts {
                match client.post(&url).send().await {
                    Ok(resp) => {
                        let status = resp.status();
                        if status.is_success() {
                            info!("Auto wallet link succeeded ({}) at {}", status, url);
                        } else {
                            let body = resp.text().await.unwrap_or_default();
                            warn!(
                                "Auto wallet link returned {} from {}: {}",
                                status, url, body
                            );
                        }
                        return;
                    }
                    Err(e) => {
                        if attempt == max_attempts {
                            error!(
                                "Auto wallet link giving up after {} attempts on {}: {}",
                                attempt, url, e
                            );
                            return;
                        }
                        debug!(
                            "Auto wallet link attempt {}/{} on {} failed ({}); retrying...",
                            attempt, max_attempts, url, e
                        );
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                }
            }
        });
    }
}
