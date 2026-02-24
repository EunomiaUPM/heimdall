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

use async_trait::async_trait;
use serde_json::Value;
use ymir::data::entities::{recv_interaction, vc_request};
use ymir::errors::Outcome;
use ymir::types::gnap::grant_request::{GrantRequest, Interact4GR};
use ymir::types::gnap::grant_response::GrantResponse;
use ymir::types::vcs::VcType;

#[async_trait]
pub trait GateKeeperTrait: Send + Sync + 'static {
    fn start(
        &self,
        grant_request: &GrantRequest
    ) -> Outcome<(vc_request::NewModel, recv_interaction::NewModel)>;
    fn validate_acc_req(&self, payload: &GrantRequest) -> Outcome<Interact4GR>;
    fn validate_vc_to_issue(&self, vc_type: &VcType) -> Outcome<()>;
    fn validate_cont_req(
        &self,
        int_model: &recv_interaction::Model,
        int_ref: String,
        token: String
    ) -> Outcome<()>;
    async fn end_verification(&self, model: recv_interaction::Model) -> Outcome<Option<String>>;
    async fn apprv_dny_req(
        &self,
        approve: bool,
        req_model: &mut vc_request::Model,
        int_model: &recv_interaction::Model
    ) -> Outcome<Value>;
    async fn notify_minion(&self, int_model: &recv_interaction::Model, body: Value) -> Outcome<()>;
    fn manage_cross_user(&self, model: recv_interaction::Model) -> Outcome<GrantResponse>;
}
