/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use std::sync::Arc;

use tracing::error;
use ymir::core_traits::CoreWalletTrait;
use ymir::errors::{ErrorLogTrait, Errors};
use ymir::services::issuer::IssuerTrait;
use ymir::services::verifier::VerifierTrait;
use ymir::services::wallet::WalletTrait;

use crate::config::CoreConfigTrait;
use crate::core::traits::{
    CoreApproverTrait, CoreGatekeeperTrait, CoreIssuerTrait, CoreTrait, CoreVerifierTrait
};
use crate::services::gatekeeper::GateKeeperTrait;
use crate::services::repo::RepoTrait;
use crate::services::vcs_builder::VcBuilderTrait;

pub struct Core {
    wallet: Option<Arc<dyn WalletTrait>>,
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
        gatekeeper: Arc<dyn GateKeeperTrait>,
        issuer: Arc<dyn IssuerTrait>,
        verifier: Arc<dyn VerifierTrait>,
        vc_builder: Arc<dyn VcBuilderTrait>,
        repo: Arc<dyn RepoTrait>,
        config: Arc<dyn CoreConfigTrait>
    ) -> Self {
        Self { wallet, gatekeeper, issuer, verifier, vc_builder, repo, config }
    }
}

impl CoreTrait for Core {
    fn config(&self) -> Arc<dyn CoreConfigTrait> { self.config.clone() }
}
impl CoreVerifierTrait for Core {
    fn verifier(&self) -> Arc<dyn VerifierTrait> { self.verifier.clone() }

    fn repo(&self) -> Arc<dyn RepoTrait> { self.repo.clone() }
}

impl CoreIssuerTrait for Core {
    fn issuer(&self) -> Arc<dyn IssuerTrait> { self.issuer.clone() }
    fn repo(&self) -> Arc<dyn RepoTrait> { self.repo.clone() }
    fn vc_builder(&self) -> Arc<dyn VcBuilderTrait> { self.vc_builder.clone() }
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
}

impl CoreWalletTrait for Core {
    fn wallet(&self) -> Arc<dyn WalletTrait> {
        self.wallet.clone().unwrap_or_else(|| {
            let error = Errors::module_new("wallet");
            error!("{}", error.log());
            panic!("module wallet not implemented");
        })
    }
}
