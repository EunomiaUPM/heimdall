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

//! Heimdall as a ymir agent: config, migrations, composition and banner.

use std::sync::Arc;

use sea_orm_migration::{MigrationTrait, MigratorTrait};
use ymir::boot::BootstrapServiceTrait;
use ymir::config::traits::ApiConfigTrait;
use ymir::errors::Outcome;
use ymir::module_loader::root_context::RootContext;
use ymir::module_loader::service_composer::ServiceComposer;

use crate::config::CoreConfig;
use crate::core::CoreBuilder;
use crate::data::Migrator;
use crate::setup::composition::{
    ApproverPlane, ConsolePlane, FedCatalogPlane, GatekeeperPlane, IssuerPlane, NotifierPlane,
    ParticipantsPlane, PlaneContext, VerifierPlane, WalletPlane,
};

pub const SERVICE_NAME: &str = "heimdall";
pub const SERVICE_BIG_NAME: &str = "Heimdall Authority";

const BANNER: &str = r"
----------
 _   _  _____ ________  ________  ___   _      _     
| | | ||  ___|_   _|  \/  |  _  \/ _ \ | |    | |    
| |_| || |__   | | | .  . | | | / /_\ \| |    | |    
|  _  ||  __|  | | | |\/| | | | |  _  || |    | |    
| | | || |___ _| |_| |  | | |/ /| | | || |____| |____
\_| |_/\____/ \___/\_|  |_/___/ \_| |_/\_____/\_____/

Starting Heimdall Authority ⚡🛡️
UPM Dataspace multistack agent
Show some love on https://github.com/EunomiaUPM/heimdall

----------

";

pub struct HeimdallBoot;

#[async_trait::async_trait]
impl BootstrapServiceTrait for HeimdallBoot {
    type Config = CoreConfig;

    const MIGRATION_TABLE: &'static str = "seaql_heimdall_migrations";

    fn banner(_big_name: &str) -> String {
        BANNER.to_string()
    }

    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        Migrator::migrations()
    }

    /// One `Core` shared by every plane; each plane mounts the routers it already had.
    async fn compose(config: &CoreConfig, root: &RootContext) -> Outcome<ServiceComposer> {
        let core = Arc::new(
            CoreBuilder::from_config(config.clone(), root.vault.clone())
                .await?
                .build(),
        );
        let ctx = PlaneContext::new(core, config.get_api_version(), root.validator.clone());
        Ok(ServiceComposer::new()
            .register(WalletPlane::new(ctx.clone()))
            .register(IssuerPlane::new(ctx.clone()))
            .register(VerifierPlane::new(ctx.clone()))
            .register(GatekeeperPlane::new(ctx.clone()))
            .register(ApproverPlane::new(ctx.clone()))
            .register(ParticipantsPlane::new(ctx.clone()))
            .register(FedCatalogPlane::new(ctx.clone()))
            .register(NotifierPlane::new(ctx.clone()))
            .register(ConsolePlane::new(ctx, config.get_openapi()?)))
    }
}
