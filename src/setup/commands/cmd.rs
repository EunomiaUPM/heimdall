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
use tracing::debug;
use ymir::config::traits::HostsConfigTrait;
use ymir::config::types::HostType;
use ymir::data::seeders::MinionSeeder;
use ymir::services::vault::vault_rs::VaultService;
use ymir::services::vault::VaultTrait;

use super::env_extraction::extract_env_config;
use crate::setup::application::AuthorityApplication;
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
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = AuthorityCli::parse();
        let vault = Arc::new(VaultService::new());

        // run scripts
        match cli.command {
            AuthorityCliCommands::Start(args) => {
                let config = extract_env_config(args.env_file)?;
                AuthorityApplication::run(config, vault).await?
            }
            AuthorityCliCommands::Setup(args) => {
                vault.write_all_secrets(None).await?;
                let config = extract_env_config(args.env_file)?;
                let db_connection = vault.get_db_connection(&config).await;
                AuthorityMigration::run(&db_connection).await?;

                let did = config.did_config.did;
                let url = config.hosts.get_host(HostType::Http);
                MinionSeeder::seed(&db_connection, did, url).await?
            }
        }

        Ok(())
    }
}
