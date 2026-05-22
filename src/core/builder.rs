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

use crate::config::traits::RoleConfigTrait;
use crate::config::types::AuthorityRole;
use crate::config::{CoreApplicationConfig, CoreConfigTrait};
use crate::core::Core;
use crate::services::gatekeeper::gnap::{config::GnapConfig, GnapService};
use crate::services::notifications::{NotificationService, NotificationsTrait};
use crate::services::repo::RepoForSql;
use crate::services::repo::RepoTrait;
use crate::services::vcs_builder::clearing_house::{
    ClearingHouseAuthorityConfig, ClearingHouseAuthorityVcBuilder,
};
use crate::services::vcs_builder::dataspace_authority::{
    DataSpaceAuthorityConfig, DataSpaceAuthorityVcBuilder,
};
use crate::services::vcs_builder::legal_authority::{
    LegalAuthorityConfig, LegalAuthorityVcBuilder,
};
use crate::services::vcs_builder::{EcoAuthorityBuilder, VcBuilderTrait};
use std::sync::Arc;
use ymir::config::traits::{ApiConfigTrait, HostsConfigTrait};
use ymir::config::types::HostType;
use ymir::services::client::ClientService;
use ymir::services::issuer::basic::config::BasicIssuerConfig;
use ymir::services::issuer::basic::BasicIssuerService;
use ymir::services::vault::{VaultService, VaultTrait};
use ymir::services::verifier::basic::config::BasicVerifierConfig;
use ymir::services::verifier::basic::BasicVerifierService;
use ymir::services::wallet::walt_id::config::WaltIdConfig;
use ymir::services::wallet::walt_id::WaltIdService;
use ymir::types::dids::{DidService, DidServiceType};

pub struct CoreBuilder {
    core: Core,
}

impl CoreBuilder {
    pub async fn from_config(config: CoreApplicationConfig, vault: Arc<VaultService>) -> Self {
        // ===== ROLE → VC BUILDER =====

        let role = config.get_role();

        let vc_builder: Arc<dyn VcBuilderTrait> = match role {
            AuthorityRole::LegalAuthority => {
                let config = LegalAuthorityConfig::from(config.clone());
                Arc::new(LegalAuthorityVcBuilder::new(config))
            }
            AuthorityRole::ClearingHouse => {
                let config = ClearingHouseAuthorityConfig::from(config.clone());
                Arc::new(ClearingHouseAuthorityVcBuilder::new(config))
            }
            AuthorityRole::ClearingHouseProxy => {
                let config = LegalAuthorityConfig::from(config.clone());
                Arc::new(LegalAuthorityVcBuilder::new(config))
            }
            AuthorityRole::DataSpaceAuthority => {
                let config = DataSpaceAuthorityConfig::from(config.clone());
                Arc::new(DataSpaceAuthorityVcBuilder::new(config))
            }
            AuthorityRole::EcoAuthority => {
                let legal_config = LegalAuthorityConfig::from(config.clone());
                let legal = Arc::new(LegalAuthorityVcBuilder::new(legal_config));

                let dp_config = DataSpaceAuthorityConfig::from(config.clone());
                let dp = Arc::new(DataSpaceAuthorityVcBuilder::new(dp_config));

                let config = ClearingHouseAuthorityConfig::from(config.clone());
                let clh = Arc::new(ClearingHouseAuthorityVcBuilder::new(config));

                Arc::new(EcoAuthorityBuilder::new(legal, dp, clh))
            }
        };

        // ===== CONFIG DERIVATIONS =====

        let gnap_config = GnapConfig::from(config.clone());
        let issuer_config = BasicIssuerConfig::from(config.clone());
        let verifier_config = BasicVerifierConfig::from(config.clone());
        let core_config: Arc<dyn CoreConfigTrait> = Arc::new(config.clone());
        let wallet_config = WaltIdConfig::from(config.clone());

        // ===== SERVICES =====

        let db_connection = vault.get_db_connection(&config).await;
        let repo: Arc<dyn RepoTrait> = Arc::new(RepoForSql::new(db_connection));

        let client = Arc::new(ClientService::default());

        let gatekeeper = Arc::new(GnapService::new(gnap_config, client.clone()));
        let issuer = Arc::new(BasicIssuerService::new(issuer_config, vault.clone()));
        let verifier = Arc::new(BasicVerifierService::new(client.clone(), verifier_config));

        let services = vec![
            DidService::basic(
                DidServiceType::CredentialIssuer,
                format!(
                    "{}{}/gate/access",
                    config.get_host(HostType::Http),
                    config.get_api_version()
                ),
            ),
            DidService::basic(
                DidServiceType::FederatedCatalog,
                format!(
                    "{}/.well-known/federated-catalog",
                    config.get_host(HostType::Http),
                ),
            ),
        ];

        let wallet = Arc::new(WaltIdService::new(wallet_config, vault, services));

        let notifier: Option<Arc<dyn NotificationsTrait>> = if config.is_react() {
            Some(Arc::new(NotificationService::new()))
        } else {
            None
        };

        let core = Core::new(
            wallet,
            notifier,
            gatekeeper,
            issuer,
            verifier,
            vc_builder,
            repo,
            core_config,
        );

        Self { core }
    }

    pub fn build(self) -> Core {
        self.core
    }
}
