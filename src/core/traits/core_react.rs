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
use std::pin::Pin;
use std::sync::Arc;

use axum::response::sse::Event;
use futures_util::Stream;

use crate::services::notifications::NotificationsTrait;

pub trait CoreReactTrait: Send + Sync + 'static {
    fn notifier(&self) -> Arc<dyn NotificationsTrait>;

    fn handle(&self) -> Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>> {
        self.notifier().handle()
    }
}
