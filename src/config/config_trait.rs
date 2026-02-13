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

use crate::types::role::AuthorityRole;
use ymir::config::traits::{
    ApiConfigTrait, ConnectionConfigTrait, DatabaseConfigTrait, HostsConfigTrait,
};
use ymir::config::types::{DidConfig, IssueConfig, VcConfig, VerifyReqConfig, WalletConfig};

pub trait CoreConfigTrait:
    HostsConfigTrait
    + ConnectionConfigTrait
    + ApiConfigTrait
    + DatabaseConfigTrait
    + Send
    + Sync
    + 'static
{
    fn get_role(&self) -> AuthorityRole;
    fn is_wallet_active(&self) -> bool;
    fn get_wallet_config(&self) -> &WalletConfig;
    fn get_did_config(&self) -> &DidConfig;
    fn get_issue_config(&self) -> &IssueConfig;
    fn get_verify_req_config(&self) -> &VerifyReqConfig;
    fn get_vc_config(&self) -> &VcConfig;
}
