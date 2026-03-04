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

use std::convert::Infallible;
use std::sync::Arc;

use axum::extract::State;
use axum::response::sse::{Event, Sse};
use axum::routing::get;
use axum::Router;
use futures_util::stream::Stream;
use serde::Serialize;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

use crate::core::traits::{CoreReactTrait, CoreTrait};

#[derive(Clone, Debug, Serialize)]
pub struct NotificationEvent {
    pub id: String,
    pub title: String,
    pub message: String,
    pub level: String,
    pub created_at: String,
    pub link: Option<String>,
}

pub struct ReactRouter {
    core: Arc<dyn CoreTrait>,
}

impl ReactRouter {
    pub fn new(core: Arc<dyn CoreTrait>) -> Self {
        Self { core }
    }

    pub fn router(self) -> Router {
        if !self.core.config().is_react() {
            return Router::new();
        }

        Router::new()
            .route("/notifications/stream", get(Self::sse_handler))
            .with_state(self.core.clone())
    }

    async fn sse_handler(
        State(core): State<Arc<dyn CoreTrait>>,
    ) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
        let rx = core.notification_sender().subscribe();
        let stream = BroadcastStream::new(rx).filter_map(|msg| match msg {
            Ok(notification) => {
                let data = serde_json::to_string(&notification).unwrap_or_default();
                Some(Ok(Event::default().data(data)))
            }
            Err(_) => None,
        });

        Sse::new(stream).keep_alive(
            axum::response::sse::KeepAlive::new().interval(std::time::Duration::from_secs(15)),
        )
    }
}
