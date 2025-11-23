/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use super::{
    m20250403_094651_interaction, m20250403_094651_issuing, m20250403_094651_minions,
    m20250403_094651_request, m20250403_094651_verification,
};
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250403_094651_request::Migration),
            Box::new(m20250403_094651_interaction::Migration),
            Box::new(m20250403_094651_verification::Migration),
            Box::new(m20250403_094651_issuing::Migration),
            Box::new(m20250403_094651_minions::Migration),
        ]
    }
}
