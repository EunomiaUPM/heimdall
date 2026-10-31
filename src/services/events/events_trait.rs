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

use async_trait::async_trait;
use ymir::data::entities::received::{grant, interaction, verification};
use ymir::data::entities::shared::{issuance, participant};

/// One event per write, under `heimdall:<entity>:<action>`, owned like the grant it belongs to.
#[async_trait]
pub trait EventsTrait: Send + Sync + 'static {
    async fn grant(&self, grant: &grant::Model, action: &str);
    async fn issuance(&self, grant: &grant::Model, issuance: &issuance::Model, action: &str);
    async fn interaction(
        &self,
        grant: &grant::Model,
        interaction: &interaction::Model,
        action: &str,
    );
    async fn verification(
        &self,
        grant: &grant::Model,
        verification: &verification::Model,
        action: &str,
    );
    async fn participant(
        &self,
        grant: &grant::Model,
        participant: &participant::Model,
        action: &str,
    );
}
