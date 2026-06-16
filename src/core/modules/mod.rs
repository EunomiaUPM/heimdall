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

mod approver;
mod fed_catalog;
mod gaia;
mod gatekeeper;
mod issuer;
mod minion;
mod orchestrator;
mod react;
mod verifier;

pub use approver::ApproverModuleTrait;
pub use fed_catalog::FedCatalogModuleTrait;
pub use gatekeeper::GatekeeperModuleTrait;
pub use issuer::IssuerModuleTrait;
pub use minion::MinionModuleTrait;
pub use orchestrator::OrchestratorTrait;
pub use react::NotifierModuleTrait;
pub use verifier::VerifierModuleTrait;
