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

use crate::services::gatekeeper::GateKeeperTrait;
use crate::services::notifications::NotificationsTrait;
use crate::services::repo::RepoTrait;
use crate::services::vcs_builder::VcBuilderTrait;
use std::sync::Arc;

pub trait HasRepo {
    fn repo(&self) -> Arc<dyn RepoTrait>;
}

pub trait HasGateKeeper {
    fn gatekeeper(&self) -> Arc<dyn GateKeeperTrait>;
}

pub trait HasVcBuilder {
    fn vc_builder(&self) -> Arc<dyn VcBuilderTrait>;
}

pub trait HasNotifier {
    fn frontend_notifier(&self) -> Arc<dyn NotificationsTrait>;
}
