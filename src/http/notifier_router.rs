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

use std::convert::Infallible;
use std::sync::Arc;

use axum::extract::State;
use axum::response::sse::{Event, Sse};
use axum::routing::get;
use axum::Router;
use futures_util::stream::Stream;

use crate::modules::FrontendNotifierModule;

pub struct ReactRouter {
    notifier: Arc<dyn FrontendNotifierModule>,
}

impl ReactRouter {
    pub fn new(notifier: Arc<dyn FrontendNotifierModule>) -> Self {
        Self { notifier }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/notifications/stream", get(Self::sse_handler))
            .with_state(self.notifier.clone())
    }

    async fn sse_handler(
        State(notificator): State<Arc<dyn FrontendNotifierModule>>,
    ) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
        let stream = notificator.handle();

        Sse::new(stream).keep_alive(
            axum::response::sse::KeepAlive::new().interval(std::time::Duration::from_secs(15)),
        )
    }
}
