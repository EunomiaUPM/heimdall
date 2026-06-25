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
use crate::services::gatekeeper::gnap::{GnapConfig, GnapGateKeeperService};
use crate::services::notifications::NotificationService;
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
use ymir::config::traits::{ApiConfigTrait, HostsConfigTrait, WalletConfigTrait};
use ymir::config::types::HostType;
use ymir::data::entities::shared::participant;
use ymir::errors::{Errors, Outcome};
use ymir::services::issuer::oid4vci_1_0;
use ymir::services::vault::{VaultService, VaultTrait};
use ymir::services::verifier::oid4vp_draft20;
use ymir::services::wallet::fafnir::{FafnirConfig, FafnirService};
use ymir::services::wallet::WalletTrait;
use ymir::types::dids::{DidService, DidServiceType};
use ymir::types::participants::ParticipantType;
use ymir::types::wallet::WalletInstance;

pub struct CoreBuilder {
    core: Core,
}

impl CoreBuilder {
    pub async fn from_config(
        config: CoreApplicationConfig,
        vault: Arc<VaultService>,
    ) -> Outcome<Self> {
        // ===== CONFIG DERIVATIONS =====

        let gnap_config = GnapConfig::from(&config);
        let issuer_config = oid4vci_1_0::IssuerConfig::from(&config);
        let verifier_config = oid4vp_draft20::VerifierConfig::from(&config);

        // ===== SERVICES =====
        let db_connection = vault.get_db_connection(&config).await?;
        let repo: Arc<dyn RepoTrait> = Arc::new(RepoForSql::new(db_connection));

        let vc_builder = Self::vc_builder(&config);
        let wallet = Self::wallet(&config, vault.clone()).await?;
        let arc_identity = wallet.get_identity();

        let identity = arc_identity.read().await;
        let participant_id = identity.did().id().to_string();

        let myself = participant::Plan {
            participant_id,
            participant_nick: "Myself".to_string(),
            participant_type: ParticipantType::Authority,
            base_url: config.get_host(HostType::Http),
            token: None,
            extra_fields: None,
            is_me: true,
        };
        repo.participant().force_update(myself).await?;

        let gatekeeper = Arc::new(GnapGateKeeperService::new(gnap_config));
        let issuer = Arc::new(oid4vci_1_0::IssuerService::new(
            issuer_config,
            vault.clone(),
            arc_identity.clone(),
        ));
        let verifier = Arc::new(oid4vp_draft20::VerifierService::new(verifier_config));
        let notifier = Arc::new(NotificationService::new());

        let core_config: Arc<dyn CoreConfigTrait> = Arc::new(config);

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

        Ok(Self { core })
    }

    pub fn build(self) -> Core {
        self.core
    }
}

impl CoreBuilder {
    fn vc_builder(config: &CoreApplicationConfig) -> Arc<dyn VcBuilderTrait> {
        let role = config.get_role();
        match role {
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
        }
    }

    async fn wallet(
        config: &CoreApplicationConfig,
        vault: Arc<VaultService>,
    ) -> Outcome<Arc<dyn WalletTrait>> {
        let services = Self::authority_services(config);
        match config.get_wallet() {
            WalletInstance::WaltId => {
                Err(Errors::not_impl("Waltid is a legacy option", None))
                // let walt_id_config = WaltIdConfig::from(config);
                // let wallet = WaltIdService::new(
                //     walt_id_config,
                //     vault.clone(),
                //     services,
                //     ParticipantType::Authority,
                // )
                // .await?;
                //
                // Ok(Arc::new(wallet))
            }
            WalletInstance::Fafnir => {
                let fafnir_config = FafnirConfig::from(config);
                let fafnir =
                    FafnirService::new(fafnir_config, vault.clone(), services.clone()).await?;
                Ok(Arc::new(fafnir))
            }
        }
    }

    fn authority_services(config: &CoreApplicationConfig) -> Vec<DidService> {
        vec![
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
        ]
    }
}
