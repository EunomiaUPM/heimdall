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

use crate::types::GrantResponseManager;
use async_trait::async_trait;
use axum::body::Bytes;
use axum::http::HeaderMap;
use ymir::data::entities::received::{grant, interaction};
use ymir::data::entities::shared::participant;
use ymir::errors::Outcome;
use ymir::types::gnap::grant_request::client::Client;
use ymir::types::gnap::grant_request::interact::InteractRequest;
use ymir::types::gnap::grant_request::GrantRequest;
use ymir::types::gnap::InteractionFinishResponse;

#[async_trait]
pub trait GateKeeperTrait: Send + Sync + 'static {
    fn build_grant_plan(&self, class_id: Option<String>) -> Outcome<grant::Plan>;
    fn build_interaction_plan(
        &self,
        id: &str,
        interact: Option<InteractRequest>,
        client: Client,
    ) -> Outcome<interaction::Plan>;
    fn build_minion_plan(&self, holder: &str, nick: &str, base_url: &str) -> participant::Plan;
    fn validate_grant(&self, payload: &Bytes, headers: &HeaderMap) -> Outcome<GrantRequest>;

    fn validate_cont_req(
        &self,
        interaction: &interaction::Model,
        payload: &Bytes,
        headers: &HeaderMap,
    ) -> Outcome<()>;
    fn manage_grant(&self, interact: Option<&InteractRequest>) -> GrantResponseManager;
    async fn finish_interaction(
        &self,
        interaction: &interaction::Model,
        verification_result: Outcome<()>,
    ) -> Outcome<InteractionFinishResponse>;
}
