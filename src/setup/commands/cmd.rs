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

use std::cmp::PartialEq;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use tracing::{debug, info};
use ymir::config::traits::ConnectionConfigTrait;
use ymir::errors::{Errors, Outcome};
use ymir::services::vault::fake_vault::FakeVaultService;
use ymir::services::vault::vault_rs::RealVaultService;
use ymir::services::vault::{VaultService, VaultTrait};

use super::env_extraction::extract_env_config;
use crate::config::CoreApplicationConfig;
use crate::setup::app::AuthorityApp;
use crate::setup::db_migrations::AuthorityMigration;

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Authority Server")]
#[command(version = "0.1")]
struct AuthorityCli {
    #[command(subcommand)]
    command: AuthorityCliCommands
}

#[derive(Parser, Debug, PartialEq)]
pub struct AuthCliArgs {
    #[arg(short, long)]
    env_file: String
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum AuthorityCliCommands {
    Start(AuthCliArgs),
    Setup(AuthCliArgs)
}

pub struct AuthorityCommands;

impl AuthorityCommands {
    pub async fn init_command_line() -> Outcome<()> {
        debug!("Init the command line application");
        let cli = AuthorityCli::parse();

        match cli.command {
            AuthorityCliCommands::Start(args) => {
                let (config, vault) = Self::bootstrap(args)?;
                AuthorityApp::run(config, Arc::new(vault)).await?
            }
            AuthorityCliCommands::Setup(args) => {
                let (config, vault) = Self::bootstrap(args)?;
                match config.is_prod() {
                    true => vault.write_all_secrets(None).await?,
                    false => vault.write_local_secrets(None).await?
                }
                let db_connection = vault.get_db_connection(&config).await;
                AuthorityMigration::run(&db_connection).await?;
            }
        }

        Ok(())
    }

    fn bootstrap(args: AuthCliArgs) -> Outcome<(CoreApplicationConfig, VaultService)> {
        let config = extract_env_config(args.env_file)?;
        let vault = if config.is_vault_real() {
            VaultService::Real(RealVaultService::new())
        } else {
            VaultService::Fake(FakeVaultService::new())
        };
        let table = json_to_table::json_to_table(
            &serde_json::to_value(&config)
                .map_err(|e| Errors::parse("Error with config table", Some(Box::new(e))))?
        )
        .collapse()
        .to_string();
        info!("Current Heimdall Config Config:\n{}", table);
        Ok((config, vault))
    }
}
