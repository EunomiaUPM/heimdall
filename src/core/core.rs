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

use ymir::core_traits::CoreWalletTrait;
use ymir::services::issuer::IssuerTrait;
use ymir::services::repo::subtraits::{MatesTrait, MinionsTrait};
use ymir::services::verifier::VerifierTrait;
use ymir::services::wallet::WalletTrait;

use crate::config::CoreConfigTrait;
use crate::core::traits::{
    CoreApproverTrait, CoreGatekeeperTrait, CoreIssuerTrait, CoreMinionTrait, CoreReactTrait,
    CoreTrait, CoreVerifierTrait
};
use crate::services::gatekeeper::GateKeeperTrait;
use crate::services::notifications::NotificationsTrait;
use crate::services::repo::RepoTrait;
use crate::services::vcs_builder::VcBuilderTrait;

pub struct Core {
    wallet: Option<Arc<dyn WalletTrait>>,
    notifier: Option<Arc<dyn NotificationsTrait>>,
    gatekeeper: Arc<dyn GateKeeperTrait>,
    issuer: Arc<dyn IssuerTrait>,
    verifier: Arc<dyn VerifierTrait>,
    vc_builder: Arc<dyn VcBuilderTrait>,
    repo: Arc<dyn RepoTrait>,
    config: Arc<dyn CoreConfigTrait>
}

impl Core {
    pub fn new(
        wallet: Option<Arc<dyn WalletTrait>>,
        notifier: Option<Arc<dyn NotificationsTrait>>,
        gatekeeper: Arc<dyn GateKeeperTrait>,
        issuer: Arc<dyn IssuerTrait>,
        verifier: Arc<dyn VerifierTrait>,
        vc_builder: Arc<dyn VcBuilderTrait>,
        repo: Arc<dyn RepoTrait>,
        config: Arc<dyn CoreConfigTrait>
    ) -> Self {
        Self { wallet, gatekeeper, issuer, verifier, vc_builder, repo, config, notifier }
    }
}

impl CoreTrait for Core {
    fn config(&self) -> Arc<dyn CoreConfigTrait> { self.config.clone() }
}

impl CoreReactTrait for Core {
    fn notifier(&self) -> Arc<dyn NotificationsTrait> {
        self.notifier.as_ref().cloned().expect("Notifier module is required for this operation but is not active in the current configuration")
    }
}

impl CoreMinionTrait for Core {
    fn repo(&self) -> Arc<dyn RepoTrait> { self.repo.clone() }
}

impl CoreVerifierTrait for Core {
    fn verifier(&self) -> Arc<dyn VerifierTrait> { self.verifier.clone() }

    fn repo(&self) -> Arc<dyn RepoTrait> { self.repo.clone() }
}

impl CoreIssuerTrait for Core {
    fn issuer(&self) -> Arc<dyn IssuerTrait> { self.issuer.clone() }
    fn repo(&self) -> Arc<dyn RepoTrait> { self.repo.clone() }
    fn vc_builder(&self) -> Arc<dyn VcBuilderTrait> { self.vc_builder.clone() }

    fn wallet(&self) -> Option<Arc<dyn WalletTrait>> { self.wallet.clone() }
}

impl CoreApproverTrait for Core {
    fn gatekeeper(&self) -> Arc<dyn GateKeeperTrait> { self.gatekeeper.clone() }

    fn repo(&self) -> Arc<dyn RepoTrait> { self.repo.clone() }
}

impl CoreGatekeeperTrait for Core {
    fn gatekeeper(&self) -> Arc<dyn GateKeeperTrait> { self.gatekeeper.clone() }

    fn verifier(&self) -> Arc<dyn VerifierTrait> { self.verifier.clone() }

    fn issuer(&self) -> Arc<dyn IssuerTrait> { self.issuer.clone() }

    fn repo(&self) -> Arc<dyn RepoTrait> { self.repo.clone() }

    fn vc_builder(&self) -> Arc<dyn VcBuilderTrait> { self.vc_builder.clone() }

    fn notifier(&self) -> Option<Arc<dyn NotificationsTrait>> { self.notifier.as_ref().cloned() }
}

impl CoreWalletTrait for Core {
    fn wallet(&self) -> Arc<dyn WalletTrait> {
        self.wallet
            .as_ref()
            .cloned()
            .expect("Wallet module is required for this operation but is not active in the current configuration")
    }

    fn mate(&self) -> Option<Arc<dyn MatesTrait>> { None }

    fn minion(&self) -> Option<Arc<dyn MinionsTrait>> { Some(self.repo.minions().clone()) }
}
