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
use tracing::{info, warn};

use crate::config::{CoreApplicationConfig, CoreConfigTrait};
use crate::core::Core;
use crate::http::RainbowAuthorityRouter;
use crate::services::client::basic::BasicClientService;
use crate::services::gatekeeper::gnap::{config::GnapConfig, GnapService};
use crate::services::issuer::basic::{config::BasicIssuerConfig, BasicIssuerService};
use crate::services::repo::postgres::RepoForSql;
use crate::services::vault::vault_rs::VaultService;
use crate::services::vault::VaultTrait;
use crate::services::vcs_builder::dataspace_authority::config::DataSpaceAuthorityConfig;
use crate::services::vcs_builder::dataspace_authority::DataSpaceAuthorityBuilder;
use crate::services::vcs_builder::legal_authority::{
    config::LegalAuthorityConfig, LegalAuthorityBuilder,
};
use crate::services::vcs_builder::VcBuilderTrait;
use crate::services::verifier::basic::{config::BasicVerifierConfig, BasicVerifierService};
use crate::services::wallet::walt_id::{config::WaltIdConfig, WaltIdService};
use crate::services::wallet::WalletTrait;
use crate::types::enums::role::AuthorityRole;
use crate::types::secrets::PemHelper;
use crate::utils::expect_from_env;

pub struct AuthorityApplication;

impl AuthorityApplication {
    pub async fn create_router(config: &CoreApplicationConfig, vault: VaultService) -> Router {
        // ROLE
        let role = config.get_role();

        let builder: Arc<dyn VcBuilderTrait> = match role {
            AuthorityRole::LegalAuthority => {
                let config = LegalAuthorityConfig::from(config.clone());
                Arc::new(LegalAuthorityBuilder::new(config))
            }
            AuthorityRole::ClearingHouse => {
                // TODO
                let config = LegalAuthorityConfig::from(config.clone());
                Arc::new(LegalAuthorityBuilder::new(config))
            }
            AuthorityRole::ClearingHouseProxy => {
                // TODO
                let config = LegalAuthorityConfig::from(config.clone());
                Arc::new(LegalAuthorityBuilder::new(config))
            }
            AuthorityRole::DataSpaceAuthority => {
                let config = DataSpaceAuthorityConfig::from(config.clone());
                Arc::new(DataSpaceAuthorityBuilder::new(config))
            }
        };

        // CONFIGS
        let gnap_config = GnapConfig::from(config.clone());
        let issuer_config = BasicIssuerConfig::from(config.clone());
        let verifier_config = BasicVerifierConfig::from(config.clone());
        let builder_service = builder;
        let core_config = Arc::new(config.clone());

        // SERVICES
        let vault = Arc::new(vault);
        let db_connection = vault.get_connection(config).await;
        let repo = Arc::new(RepoForSql::new(db_connection));
        let client = Arc::new(BasicClientService::new());
        let access = Arc::new(GnapService::new(gnap_config, client.clone()));
        let issuer = Arc::new(BasicIssuerService::new(issuer_config, vault.clone()));
        let verifier = Arc::new(BasicVerifierService::new(client.clone(), verifier_config));

        let wallet: Option<Arc<dyn WalletTrait>> = match config.is_wallet_active() {
            true => {
                let walt_id_config = WaltIdConfig::from(config.clone());
                Some(Arc::new(WaltIdService::new(walt_id_config, client.clone(), vault)))
            }
            false => None,
        };

        // CORE
        let core = Core::new(wallet, access, issuer, verifier, builder_service, repo, core_config);

        // ROUTER
        RainbowAuthorityRouter::new(Arc::new(core)).router()
    }

    pub async fn run_basic(
        config: CoreApplicationConfig,
        vault: VaultService,
    ) -> anyhow::Result<()> {
        let router = Self::create_router(&config, vault).await;

        let server_message = format!("Starting Authority server in {}", config.get_host());
        info!("{}", server_message);

        let listener = match config.is_local() {
            true => TcpListener::bind(format!("127.0.0.1{}", config.get_weird_port())).await?,
            false => TcpListener::bind(format!("0.0.0.0{}", config.get_weird_port())).await?,
        };

        serve(listener, router).await?;
        Ok(())
    }
    pub async fn run_tls(
        config: &CoreApplicationConfig,
        vault: VaultService,
    ) -> anyhow::Result<()> {
        let cert = expect_from_env("VAULT_CLIENT_CERT");
        let pkey = expect_from_env("VAULT_CLIENT_KEY");
        let cert: PemHelper = vault.read(None, &cert).await?;
        let pkey: PemHelper = vault.read(None, &pkey).await?;

        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("No se pudo instalar el proveedor de criptografía");

        let tls_config = RustlsConfig::from_pem(cert.data().to_vec(), pkey.data().to_vec()).await?;

        let router = Self::create_router(config, vault).await;

        let addr_str =
            if config.is_local() { format!("127.0.0.1:443") } else { format!("0.0.0.0:443") };
        let addr: SocketAddr = addr_str.parse()?;
        info!("Starting Authority server with TLS in {}", addr);

        axum_server::bind_rustls(addr, tls_config).serve(router.into_make_service()).await?;
        Ok(())
    }
    pub async fn run(config: CoreApplicationConfig, vault: VaultService) -> anyhow::Result<()> {
        match Self::run_tls(&config, vault.clone()).await {
            Ok(_) => Ok(()),
            Err(err) => {
                warn!("TLS failed: {:?}, falling back to basic server", err);
                Self::run_basic(config, vault).await
            }
        }
    }
}
