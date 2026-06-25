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
mod notifier;
mod participants;
mod verifier;

pub use crate::core::OrchestratorTrait;
pub use approver::ApproverModule;
pub use fed_catalog::FedCatalogModule;
pub use gatekeeper::GatekeeperModule;
pub use issuer::IssuerModule;
pub use notifier::FrontendNotifierModule;
pub use participants::ParticipantModule;
pub use verifier::VerifierModule;
