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

use async_trait::async_trait;
use chrono::Utc;
use rand::Rng;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, SqlErr,
};
use ymir::errors::{Errors, Outcome};
use ymir::types::vcs::doc::StatusPurpose;
use ymir::types::vcs::MIN_STATUS_LIST_BITS;

use crate::data::entities::vc_status::{ActiveModel, Column, Entity, Model, Plan};

pub const STATUS_LIST_CAPACITY: i32 = MIN_STATUS_LIST_BITS as i32;
const MAX_FILL: u64 = (STATUS_LIST_CAPACITY / 2) as u64;
const MAX_TRIES: usize = 32;

#[async_trait]
pub trait VcStatusRepoTrait: Send + Sync + 'static {
    async fn allocate(&self, plan: Plan) -> Outcome<Model>;
    async fn get_all(&self) -> Outcome<Vec<Model>>;
    async fn get_by_issuance(&self, issuance_id: &str) -> Outcome<Model>;
    async fn last_list_no(&self) -> Outcome<i32>;
    async fn marked_indices(&self, list_no: i32, purpose: StatusPurpose) -> Outcome<Vec<i32>>;
    async fn update(&self, model: Model) -> Outcome<Model>;
}

pub struct VcStatusPostgresRepo {
    db: DatabaseConnection,
}

impl VcStatusPostgresRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    async fn insert(&self, plan: &Plan, list_no: i32, idx: i32) -> Result<Model, DbErr> {
        ActiveModel {
            credential_id: ActiveValue::Set(plan.credential_id.clone()),
            issuance_id: ActiveValue::Set(plan.issuance_id.clone()),
            vc_type: ActiveValue::Set(plan.vc_type.clone()),
            holder_did: ActiveValue::Set(plan.holder_did.clone()),
            subject_name: ActiveValue::Set(plan.subject_name.clone()),
            list_no: ActiveValue::Set(list_no),
            idx: ActiveValue::Set(idx),
            issued_at: ActiveValue::Set(Utc::now()),
            revoked_at: ActiveValue::Set(None),
            suspended_at: ActiveValue::Set(None),
        }
        .insert(&self.db)
        .await
    }

    async fn find_by_credential(&self, credential_id: &str) -> Outcome<Option<Model>> {
        Entity::find_by_id(credential_id.to_string())
            .one(&self.db)
            .await
            .map_err(db_err)
    }
}

#[async_trait]
impl VcStatusRepoTrait for VcStatusPostgresRepo {
    async fn allocate(&self, plan: Plan) -> Outcome<Model> {
        if let Some(model) = self.find_by_credential(&plan.credential_id).await? {
            return Ok(model);
        }

        let mut list_no = self.last_list_no().await?;
        let used = Entity::find()
            .filter(Column::ListNo.eq(list_no))
            .count(&self.db)
            .await
            .map_err(db_err)?;
        if used >= MAX_FILL {
            list_no += 1;
        }

        for _ in 0..MAX_TRIES {
            let idx = rand::thread_rng().gen_range(0..STATUS_LIST_CAPACITY);
            match self.insert(&plan, list_no, idx).await {
                Ok(model) => return Ok(model),
                Err(e) if matches!(e.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) => {
                    if let Some(model) = self.find_by_credential(&plan.credential_id).await? {
                        return Ok(model);
                    }
                }
                Err(e) => return Err(db_err(e)),
            }
        }
        Err(Errors::db("Unable to allocate a status list index", None))
    }

    async fn get_all(&self) -> Outcome<Vec<Model>> {
        Entity::find()
            .order_by_desc(Column::IssuedAt)
            .all(&self.db)
            .await
            .map_err(db_err)
    }

    async fn get_by_issuance(&self, issuance_id: &str) -> Outcome<Model> {
        Entity::find()
            .filter(Column::IssuanceId.eq(issuance_id))
            .one(&self.db)
            .await
            .map_err(db_err)?
            .ok_or_else(|| {
                Errors::missing_resource(issuance_id, "No issued credential for this id", None)
            })
    }

    async fn last_list_no(&self) -> Outcome<i32> {
        let last: Option<Option<i32>> = Entity::find()
            .select_only()
            .column_as(Column::ListNo.max(), "list_no")
            .into_tuple()
            .one(&self.db)
            .await
            .map_err(db_err)?;
        Ok(last.flatten().unwrap_or(0))
    }

    async fn marked_indices(&self, list_no: i32, purpose: StatusPurpose) -> Outcome<Vec<i32>> {
        let marked = match purpose {
            StatusPurpose::Revocation => Column::RevokedAt.is_not_null(),
            StatusPurpose::Suspension => Column::SuspendedAt.is_not_null(),
            _ => return Ok(vec![]),
        };
        Entity::find()
            .select_only()
            .column(Column::Idx)
            .filter(Column::ListNo.eq(list_no))
            .filter(marked)
            .into_tuple()
            .all(&self.db)
            .await
            .map_err(db_err)
    }

    async fn update(&self, model: Model) -> Outcome<Model> {
        let mut active = model.clone().into_active_model();
        active.revoked_at = ActiveValue::Set(model.revoked_at);
        active.suspended_at = ActiveValue::Set(model.suspended_at);
        active.update(&self.db).await.map_err(db_err)
    }
}

fn db_err(e: DbErr) -> Errors {
    Errors::db("Error accessing vc_status", Some(Box::new(e)))
}
