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
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::routing::post;
use axum::Router;
use ymir::errors::AppResult;
use ymir::http_tracing::ExtractedHeaders;
use ymir::types::vcs::VcTypeConfig;

use crate::http::routes::gaia;
use crate::modules::GaiaModule;

/// The type issued when the path names none.
const DEFAULT_TYPE: &str = "gx_LabelCredential_jwt_vc_json";

pub struct GaiaRouter {
    gaia: Arc<dyn GaiaModule>,
}

impl GaiaRouter {
    pub fn new(gaia: Arc<dyn GaiaModule>) -> Self {
        Self { gaia }
    }

    /// Open: the holder is authenticated by the signature of its presentation.
    pub fn external(self) -> Router {
        Router::new()
            .route(gaia::COMPLIANCE, post(Self::compliance))
            .route(gaia::COMPLIANCE_BY_TYPE, post(Self::compliance_by_type))
            .with_state(self.gaia)
    }

    async fn compliance(
        State(gaia): State<Arc<dyn GaiaModule>>,
        trace: ExtractedHeaders,
        Query(params): Query<HashMap<String, String>>,
        body: String,
    ) -> AppResult<(StatusCode, HeaderMap, String)> {
        Self::issue(gaia, trace, DEFAULT_TYPE, params, body).await
    }

    async fn compliance_by_type(
        State(gaia): State<Arc<dyn GaiaModule>>,
        trace: ExtractedHeaders,
        Path(vc_type): Path<String>,
        Query(params): Query<HashMap<String, String>>,
        body: String,
    ) -> AppResult<(StatusCode, HeaderMap, String)> {
        Self::issue(gaia, trace, &vc_type, params, body).await
    }

    async fn issue(
        gaia: Arc<dyn GaiaModule>,
        trace: ExtractedHeaders,
        vc_type: &str,
        params: HashMap<String, String>,
        body: String,
    ) -> AppResult<(StatusCode, HeaderMap, String)> {
        let Ok(vc_type) = VcTypeConfig::from_str(vc_type);
        let vcid = params.get("vcid").cloned();
        let vc = gaia.gaia_compliance(vc_type, vcid, body.trim()).await?;
        let mut headers = trace.response_headers();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/vc+jwt"));
        Ok((StatusCode::CREATED, headers, vc))
    }
}
