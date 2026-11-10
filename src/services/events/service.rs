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

use ::events::EventSinkTrait;
use async_trait::async_trait;
use ymir::data::entities::received::{grant, interaction, verification};
use ymir::data::entities::shared::{issuance, participant};
use ymir::oauth::Owner;

use crate::data::entities::vc_status;

use super::{EventsTrait, GrantEvent, InteractionEvent, IssuanceEvent, VerificationEvent};
use crate::EVENT_PREFIX;

pub struct EventsService {
    sink: Arc<dyn EventSinkTrait>,
}

impl EventsService {
    pub fn new(sink: Arc<dyn EventSinkTrait>) -> Self {
        Self { sink }
    }

    /// A received grant has no author: the role that handles it, seen as it is.
    fn owner(grant: &grant::Model) -> Owner {
        Owner::team(grant.role.clone(), grant.visibility.clone())
    }
}

#[async_trait]
impl EventsTrait for EventsService {
    async fn grant(&self, grant: &grant::Model, action: &str) {
        let payload = GrantEvent::from(grant);
        events::emit_action!(
            self.sink,
            &Self::owner(grant),
            EVENT_PREFIX,
            "grant",
            action,
            &payload
        );
    }

    async fn issuance(&self, grant: &grant::Model, issuance: &issuance::Model, action: &str) {
        let payload = IssuanceEvent::from(issuance);
        events::emit_action!(
            self.sink,
            &Self::owner(grant),
            EVENT_PREFIX,
            "issuance",
            action,
            &payload
        );
    }

    async fn interaction(
        &self,
        grant: &grant::Model,
        interaction: &interaction::Model,
        action: &str,
    ) {
        let payload = InteractionEvent::from(interaction);
        events::emit_action!(
            self.sink,
            &Self::owner(grant),
            EVENT_PREFIX,
            "interaction",
            action,
            &payload
        );
    }

    async fn verification(
        &self,
        grant: &grant::Model,
        verification: &verification::Model,
        action: &str,
    ) {
        let payload = VerificationEvent::from(verification);
        events::emit_action!(
            self.sink,
            &Self::owner(grant),
            EVENT_PREFIX,
            "verification",
            action,
            &payload
        );
    }

    async fn participant(
        &self,
        grant: &grant::Model,
        participant: &participant::Model,
        action: &str,
    ) {
        events::emit_action!(
            self.sink,
            &Self::owner(grant),
            EVENT_PREFIX,
            "participant",
            action,
            participant
        );
    }

    async fn vc_status(&self, grant: &grant::Model, status: &vc_status::Model, action: &str) {
        events::emit_action!(
            self.sink,
            &Self::owner(grant),
            EVENT_PREFIX,
            "vc_status",
            action,
            status
        );
    }
}
