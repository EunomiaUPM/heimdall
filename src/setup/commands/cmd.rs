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

use clap::{Parser, Subcommand};
use tracing::debug;

use super::env_extraction::extract_env_config;
use crate::services::vault::vault_rs::VaultService;
use crate::services::vault::VaultTrait;
use crate::setup::application::AuthorityApplication;
use crate::setup::database::db_migrations::AuthorityMigration;

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
    env_file: Option<String>
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum AuthorityCliCommands {
    Start(AuthCliArgs),
    Setup(AuthCliArgs),
    Vault
}

pub struct AuthorityCommands;

impl AuthorityCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = AuthorityCli::parse();

        // run scripts
        match cli.command {
            AuthorityCliCommands::Start(args) => {
                let config = extract_env_config(args.env_file)?;
                let vault = VaultService::new();
                AuthorityApplication::run_basic(config, vault).await?
            }
            AuthorityCliCommands::Setup(args) => {
                let config = extract_env_config(args.env_file)?;
                let vault = VaultService::new();
                let db_connection = vault.get_connection(&config).await;
                AuthorityMigration::run(db_connection).await?
            }
            AuthorityCliCommands::Vault => {
                let vault = VaultService::new();
                vault.write_all_secrets().await?
            }
        }

        Ok(())
    }
}
