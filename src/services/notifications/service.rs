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

use super::NotificationEvent;
use super::NotificationsTrait;
use axum::response::sse::Event;
use futures_util::Stream;
use futures_util::StreamExt;
use std::convert::Infallible;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio_stream::wrappers::BroadcastStream;
use ymir::data::entities::vc_request::Model;

pub struct NotificationService {
    sender: Arc<Sender<NotificationEvent>>,
}

impl NotificationService {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(100);
        let sender = Arc::new(tx);
        Self { sender }
    }
}

impl NotificationsTrait for NotificationService {
    fn notify(&self, model: &Model) {
        let event = NotificationEvent {
            id: model.id.clone(),
            title: "New Petition".to_string(),
            message: format!("{} requests a {} credential", model.participant_slug, model.vc_type),
            level: "info".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            link: Some(format!("/requests/{}", model.id)),
        };
        let _ = self.sender.send(event);
    }

    fn handle(&self) -> Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>> {
        let rx = self.sender.subscribe();

        let stream = BroadcastStream::new(rx).filter_map(|msg| async move {
            match msg {
                Ok(notification) => {
                    let data = serde_json::to_string(&notification).unwrap_or_default();
                    Some(Ok(Event::default().data(data)))
                }
                Err(_) => None,
            }
        });

        Box::pin(stream)
    }
}
