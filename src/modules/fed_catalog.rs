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
use std::time::Duration;

use crate::services::repo::RepoTrait;
use crate::services::HasRepo;
use async_trait::async_trait;
use chrono::Utc;
use serde_json::{json, Value};
use tracing::warn;
use ymir::data::entities::shared::participant::Model;
use ymir::errors::Outcome;
use ymir::services::client::ClientExt;
use ymir::types::participants::ParticipantType;
use ymir::utils::http_client;
use tokio_util::sync::CancellationToken;
use ymir::boot::workers::BackgroundWorker;

const DSP_CRAWL_SECS: u64 = 300;

#[async_trait]
pub trait FedCatalogModule: HasRepo + Send + Sync + 'static {
    #[tracing::instrument(level = "info", skip_all, err)]
    async fn get_all(&self) -> Outcome<Vec<Model>> {
        self.repo().participant().get_all(None, None).await
    }
}

/// Refreshes the DSP versions of every agent now and then every `DSP_CRAWL_INTERVAL_SECS`.
pub async fn crawl_dsp_every(repo: Arc<dyn RepoTrait>) {
    let secs = std::env::var("DSP_CRAWL_INTERVAL_SECS")
        .ok()
        .and_then(|secs| secs.parse().ok())
        .unwrap_or(DSP_CRAWL_SECS);
    let mut ticks = tokio::time::interval(Duration::from_secs(secs));
    loop {
        ticks.tick().await;
        match repo.participant().get_all(None, None).await {
            Ok(participants) => {
                for participant in participants {
                    crawl_dsp(repo.clone(), participant).await;
                }
            }
            Err(e) => e.log(),
        }
    }
}

/// [`crawl_dsp_every`] as a boot worker, stopped with the process.
pub struct DspCrawler(pub Arc<dyn RepoTrait>);

#[async_trait]
impl BackgroundWorker for DspCrawler {
    fn name(&self) -> &'static str {
        "dsp-crawler"
    }

    async fn run(self: Box<Self>, token: CancellationToken) -> Outcome<()> {
        tokio::select! {
            _ = token.cancelled() => {}
            _ = crawl_dsp_every(self.0) => {}
        }
        Ok(())
    }
}

/// Stores the agent's DSP versions in `extra_fields.dsp`; a failure keeps the last ones.
pub async fn crawl_dsp(repo: Arc<dyn RepoTrait>, mut participant: Model) {
    if participant.participant_type != ParticipantType::Agent {
        return;
    }
    let base_url = participant.base_url.trim_end_matches('/').to_string();
    let url = format!("{base_url}/.well-known/dspace-version");
    let versions: Value = match http_client().get_json(&url, None).await {
        Ok(versions) => versions,
        Err(e) => {
            warn!("No DSP versions from {url}: {e}");
            return;
        }
    };
    let base = versions["protocolVersions"][0]["path"]
        .as_str()
        .map(|path| match path.starts_with("http") {
            true => path.trim_end_matches('/').to_string(),
            false => format!("{base_url}{}", path.trim_end_matches('/')),
        });
    if !participant.extra_fields.is_object() {
        participant.extra_fields = json!({});
    }
    participant.extra_fields["dsp"] = json!({
        "base": base,
        "versions": versions["protocolVersions"],
        "checked_at": Utc::now(),
    });
    if let Err(e) = repo.participant().update(participant).await {
        e.log();
    }
}
