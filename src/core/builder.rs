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

use ymir::services::client::basic::BasicClientService;
use ymir::services::issuer::basic::config::BasicIssuerConfig;
use ymir::services::issuer::basic::BasicIssuerService;
use ymir::services::vault::{VaultService, VaultTrait};
use ymir::services::verifier::basic::config::BasicVerifierConfig;
use ymir::services::verifier::basic::BasicVerifierService;
use ymir::services::wallet::walt_id::config::WaltIdConfig;
use ymir::services::wallet::walt_id::WaltIdService;
use ymir::services::wallet::WalletTrait;

use crate::config::role::{AuthorityRole, RoleConfigTrait};
use crate::config::{CoreApplicationConfig, CoreConfigTrait};
use crate::core::Core;
use crate::services::gatekeeper::gnap::{config::GnapConfig, GnapService};
use crate::services::repo::RepoForSql;
use crate::services::repo::RepoTrait;
use crate::services::vcs_builder::dataspace_authority::{
    config::DataSpaceAuthorityConfig, DataSpaceAuthorityVcBuilder
};
use crate::services::vcs_builder::legal_authority::{
    LegalAuthorityConfig, LegalAuthorityVcBuilder
};
use crate::services::vcs_builder::{EcoAuthorityBuilder, VcBuilderTrait};

pub struct CoreBuilder {
    core: Core
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
                let config = LegalAuthorityConfig::from(config.clone());
                Arc::new(LegalAuthorityVcBuilder::new(config))
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

                Arc::new(EcoAuthorityBuilder::new(legal, dp))
            }
        };

        // ===== CONFIG DERIVATIONS =====

        let gnap_config = GnapConfig::from(config.clone());
        let issuer_config = BasicIssuerConfig::from(config.clone());
        let verifier_config = BasicVerifierConfig::from(config.clone());
        let core_config: Arc<dyn CoreConfigTrait> = Arc::new(config.clone());

        // ===== SERVICES =====

        let db_connection = vault.get_db_connection(&config).await;
        let repo: Arc<dyn RepoTrait> = Arc::new(RepoForSql::new(db_connection));

        let client = Arc::new(BasicClientService::new());

        let gatekeeper = Arc::new(GnapService::new(gnap_config, client.clone()));
        let issuer =
            Arc::new(BasicIssuerService::new(issuer_config, client.clone(), vault.clone()));
        let verifier = Arc::new(BasicVerifierService::new(client.clone(), verifier_config));

        let wallet: Option<Arc<dyn WalletTrait>> = if config.is_wallet_active() {
            let walt_config = WaltIdConfig::from(config.clone());
            Some(Arc::new(WaltIdService::new(walt_config, client.clone(), vault)))
        } else {
            None
        };

        let core = Core::new(wallet, gatekeeper, issuer, verifier, vc_builder, repo, core_config);

        Self { core }
    }

    pub fn build(self) -> Core { self.core }
}
