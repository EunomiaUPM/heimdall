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

use std::sync::Arc;

use async_trait::async_trait;

use crate::services::issuer::IssuerTrait;
use crate::services::repo::RepoTrait;
use crate::services::verifier::VerifierTrait;

#[async_trait]
pub trait GaiaCoreTrait: Send + Sync + 'static {
    fn verifier(&self) -> Arc<dyn VerifierTrait>;
    fn repo(&self) -> Arc<dyn RepoTrait>;
    fn issuer(&self) -> Arc<dyn IssuerTrait>;
    async fn manage_req(&self) -> anyhow::Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let model = self.verifier().start_vp(&id)?;
        let model = self.repo().verification().create(model).await?;
        let uri = self.verifier().generate_verification_uri(model);
        Ok(uri)
    }
    async fn verify(&self, state: String, vp_token: String) -> anyhow::Result<String> {
        let mut ver_model = self.repo().verification().get_by_state(&state).await?;
        let result = self.verifier().verify_all(&mut ver_model, vp_token).await;
        let _int_model = self.repo().interaction().get_by_id(&ver_model.id).await?;
        result?;
        self.repo().verification().update(ver_model).await?;
        // let uri = self.issuer().
        todo!()
    }
}
