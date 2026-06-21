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

use std::sync::Arc;
use ymir::modules::WalletModuleTrait;
use ymir::services::issuer::IssuerTrait;
use ymir::services::verifier::VerifierTrait;
use ymir::services::wallet::WalletTrait;
use ymir::services::{HasIssuer, HasVerifier, HasWallet};
use ymir::services::repo::traits::shared::ParticipantRepoTrait;
use crate::config::CoreConfigTrait;
use crate::modules::{
    ApproverModule, FedCatalogModule, FrontendNotifierModule, GatekeeperModule, IssuerModule,
    OrchestratorTrait, ParticipantModule, VerifierModule,
};
use crate::services::gatekeeper::GateKeeperTrait;
use crate::services::notifications::NotificationsTrait;
use crate::services::repo::RepoTrait;
use crate::services::vcs_builder::VcBuilderTrait;
use crate::services::{HasGateKeeper, HasNotifier, HasRepo, HasVcBuilder};

pub struct Core {
    wallet: Arc<dyn WalletTrait>,
    notifier: Arc<dyn NotificationsTrait>,
    gatekeeper: Arc<dyn GateKeeperTrait>,
    issuer: Arc<dyn IssuerTrait>,
    verifier: Arc<dyn VerifierTrait>,
    vc_builder: Arc<dyn VcBuilderTrait>,
    repo: Arc<dyn RepoTrait>,
    config: Arc<dyn CoreConfigTrait>,
}

impl Core {
    pub fn new(
        wallet: Arc<dyn WalletTrait>,
        notifier: Arc<dyn NotificationsTrait>,
        gatekeeper: Arc<dyn GateKeeperTrait>,
        issuer: Arc<dyn IssuerTrait>,
        verifier: Arc<dyn VerifierTrait>,
        vc_builder: Arc<dyn VcBuilderTrait>,
        repo: Arc<dyn RepoTrait>,
        config: Arc<dyn CoreConfigTrait>,
    ) -> Self {
        Self {
            wallet,
            gatekeeper,
            issuer,
            verifier,
            vc_builder,
            repo,
            config,
            notifier,
        }
    }
}

impl HasVerifier for Core {
    fn verifier(&self) -> Arc<dyn VerifierTrait> {
        self.verifier.clone()
    }
}

impl HasRepo for Core {
    fn repo(&self) -> Arc<dyn RepoTrait> {
        self.repo.clone()
    }
}

impl HasIssuer for Core {
    fn issuer(&self) -> Arc<dyn IssuerTrait> {
        self.issuer.clone()
    }
}

impl HasVcBuilder for Core {
    fn vc_builder(&self) -> Arc<dyn VcBuilderTrait> {
        self.vc_builder.clone()
    }
}

impl HasWallet for Core {
    fn wallet(&self) -> Arc<dyn WalletTrait> {
        self.wallet.clone()
    }
}

impl HasGateKeeper for Core {
    fn gatekeeper(&self) -> Arc<dyn GateKeeperTrait> {
        self.gatekeeper.clone()
    }
}

impl HasNotifier for Core {
    fn frontend_notifier(&self) -> Arc<dyn NotificationsTrait> {
        self.notifier.clone()
    }
}

impl WalletModuleTrait for Core {

    fn participant(&self) -> Arc<dyn ParticipantRepoTrait> {
        self.repo.participant()
    }
}

impl VerifierModule for Core {}
impl IssuerModule for Core {}
impl ApproverModule for Core {}
impl GatekeeperModule for Core {}
impl FedCatalogModule for Core {}
impl ParticipantModule for Core {}
impl FrontendNotifierModule for Core {}
impl OrchestratorTrait for Core {
    fn config(&self) -> Arc<dyn CoreConfigTrait> {
        self.config.clone()
    }
}
