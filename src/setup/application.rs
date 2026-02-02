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
use ymir::config::traits::HostsConfigTrait;
use ymir::config::types::HostType;
use ymir::services::client::basic::BasicClientService;
use ymir::services::issuer::basic::config::BasicIssuerConfig;
use ymir::services::issuer::basic::BasicIssuerService;
use ymir::services::vault::vault_rs::VaultService;
use ymir::services::vault::VaultTrait;
use ymir::services::verifier::basic::config::BasicVerifierConfig;
use ymir::services::verifier::basic::BasicVerifierService;
use ymir::services::wallet::walt_id::config::WaltIdConfig;
use ymir::services::wallet::walt_id::WaltIdService;
use ymir::services::wallet::WalletTrait;
use ymir::types::secrets::StringHelper;
use ymir::utils::expect_from_env;

use crate::config::{CoreApplicationConfig, CoreConfigTrait};
use crate::core::Core;
use crate::http::RainbowAuthorityRouter;
use crate::services::gatekeeper::gnap::{config::GnapConfig, GnapService};
use crate::services::repo::RepoForSql;
use crate::services::vcs_builder::dataspace_authority::config::DataSpaceAuthorityConfig;
use crate::services::vcs_builder::dataspace_authority::DataSpaceAuthorityVcBuilder;
use crate::services::vcs_builder::legal_authority::{
    config::LegalAuthorityConfig, LegalAuthorityVcBuilder
};
use crate::services::vcs_builder::VcBuilderTrait;
use crate::types::role::AuthorityRole;

pub struct AuthorityApplication;

impl AuthorityApplication {
    pub async fn create_router(config: &CoreApplicationConfig, vault: Arc<VaultService>) -> Router {
        // ROLE
        let role = config.get_role();

        let builder: Arc<dyn VcBuilderTrait> = match role {
            AuthorityRole::LegalAuthority => {
                let config = LegalAuthorityConfig::from(config.clone());
                Arc::new(LegalAuthorityVcBuilder::new(config))
            }
            AuthorityRole::ClearingHouse => {
                // TODO
                let config = LegalAuthorityConfig::from(config.clone());
                Arc::new(LegalAuthorityVcBuilder::new(config))
            }
            AuthorityRole::ClearingHouseProxy => {
                // TODO
                let config = LegalAuthorityConfig::from(config.clone());
                Arc::new(LegalAuthorityVcBuilder::new(config))
            }
            AuthorityRole::DataSpaceAuthority => {
                let config = DataSpaceAuthorityConfig::from(config.clone());
                Arc::new(DataSpaceAuthorityVcBuilder::new(config))
            }
        };

        // CONFIGS
        let gnap_config = GnapConfig::from(config.clone());
        let issuer_config = BasicIssuerConfig::from(config.clone());
        let verifier_config = BasicVerifierConfig::from(config.clone());
        let builder_service = builder;
        let core_config = Arc::new(config.clone());

        // SERVICES
        let db_connection = vault.get_db_connection(config).await;
        let repo = Arc::new(RepoForSql::new(db_connection));
        let client = Arc::new(BasicClientService::new());
        let access = Arc::new(GnapService::new(gnap_config, client.clone()));
        let issuer =
            Arc::new(BasicIssuerService::new(issuer_config, client.clone(), vault.clone()));
        let verifier = Arc::new(BasicVerifierService::new(client.clone(), verifier_config));

        let wallet: Option<Arc<dyn WalletTrait>> = match config.is_wallet_active() {
            true => {
                let walt_id_config = WaltIdConfig::from(config.clone());
                Some(Arc::new(WaltIdService::new(walt_id_config, client.clone(), vault)))
            }
            false => None
        };

        // CORE
        let core = Core::new(wallet, access, issuer, verifier, builder_service, repo, core_config);

        // ROUTER
        RainbowAuthorityRouter::new(Arc::new(core)).router()
    }

    pub async fn run_basic(
        config: CoreApplicationConfig,
        vault: Arc<VaultService>
    ) -> anyhow::Result<()> {
        let router = Self::create_router(&config, vault).await;

        let server_message = format!(
            "Starting Authority server in {}",
            config.hosts().get_host(HostType::Http)
        );
        info!("{}", server_message);

        let listener = match config.is_local() {
            true => {
                TcpListener::bind(format!(
                    "0.0.0.0{}",
                    config.hosts().get_weird_port(HostType::Http)
                ))
                .await?
            }
            false => {
                TcpListener::bind(format!(
                    "0.0.0.0{}",
                    config.hosts().get_weird_port(HostType::Http)
                ))
                .await?
            }
        };

        serve(listener, router).await?;
        Ok(())
    }
    pub async fn run_tls(
        config: &CoreApplicationConfig,
        vault: Arc<VaultService>
    ) -> anyhow::Result<()> {
        let cert = expect_from_env("VAULT_APP_ROOT_CLIENT_KEY");
        let pkey = expect_from_env("VAULT_APP_CLIENT_KEY ");
        let cert: StringHelper = vault.read(None, &cert).await?;
        let pkey: StringHelper = vault.read(None, &pkey).await?;

        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Unable to install cryptography provider");

        let tls_config = RustlsConfig::from_pem(
            cert.data().as_bytes().to_vec(),
            pkey.data().as_bytes().to_vec()
        )
        .await?;

        let router = Self::create_router(config, vault).await;

        let addr_str = if config.is_local() { "0.0.0.0:443" } else { "0.0.0.0:443" };
        let addr: SocketAddr = addr_str.parse()?;
        info!("Starting Authority server with TLS in {}", addr);

        axum_server::bind_rustls(addr, tls_config).serve(router.into_make_service()).await?;
        Ok(())
    }
    pub async fn run(
        config: CoreApplicationConfig,
        vault: Arc<VaultService>
    ) -> anyhow::Result<()> {
        if config.is_tls {
            Self::run_tls(&config, vault.clone()).await
        } else {
            Self::run_basic(config, vault).await
        }
    }
}
