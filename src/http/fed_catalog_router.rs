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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::Arc;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use ymir::data::entities::minions::Model;
use ymir::errors::AppResult;

use crate::core::modules::FedCatalogModuleTrait;

pub struct FedCatalogRouter {
    gru: Arc<dyn FedCatalogModuleTrait>,
}

impl FedCatalogRouter {
    pub fn new(gru: Arc<dyn FedCatalogModuleTrait>) -> FedCatalogRouter {
        FedCatalogRouter { gru }
    }

    // pub fn router(self) -> Router {
    //     Router::new()
    //         .route("/all", get(Self::get_all))
    //         .with_state(self.gru)
    // }

    pub fn well_known(&self) -> Router {
        Router::new()
            .route("/.well-known/federated-catalog", get(Self::get_all))
            .with_state(self.gru.clone())
    }

    async fn get_all(State(gru): State<Arc<dyn FedCatalogModuleTrait>>) -> AppResult<Json<Vec<Model>>> {
        Ok(Json(gru.get_all().await?))
    }
}
