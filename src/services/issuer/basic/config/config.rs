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

use crate::config::{CoreApplicationConfig, CoreApplicationConfigTrait};
use crate::services::issuer::basic::config::config_trait::BasicIssuerConfigTrait;
use crate::types::enums::data_model::W3cDataModelVersion;
use crate::types::host::HostConfig;
use crate::utils::read;

pub struct BasicIssuerConfig {
    host: HostConfig,
    is_local: bool,
    keys_path: String,
    api_path: String,
    w3c_vc_data_model: Option<W3cDataModelVersion>,
    did: String
}

impl From<CoreApplicationConfig> for BasicIssuerConfig {
    fn from(config: CoreApplicationConfig) -> BasicIssuerConfig {
        let api_path = config.get_api_path();
        BasicIssuerConfig {
            host: config.host,
            is_local: config.is_local,
            keys_path: config.keys_path,
            api_path,
            w3c_vc_data_model: config.stuff_to_issue.w3c_data_model,
            did: "did:jwk:eyJrdHkiOiJSU0EiLCJlIjoiQVFBQiIsIm4iOiJ4T0F3WlpkQ3AwQ0dCQV9tMVVHR1VXcEM5THJlcmJjbDMxblJoZThzNzd6R204aXA3YUgtUW5NNUNLbEo1YXlvUDhKTV9rLWhVQ0V2Q09qNWw5VmNhN0RfNTdRMjRJUEtHN0dQSGdnYzZ5bGxwZFlnSkVlY2xUcXJGcjdGUFBxcThDQzY1WVpfS1hGY3ByOHJGRVFEUGtkc3gzbU1qTkYzQUpNNzBpaEItdGtOOWxxcHZLb0h6MzJRaHRybVhFY2t1X1NtVFlFTU9GVU0xd0I1TE4xNmN4ZzIxZXYwMnZGdlVCYmdWZENTNHZocjNHNlNMM2dPZXVzd3BZaTZtTTZ6clloY3lkaDJaRjZCUEZVMS1NNk9tcENGRnd6SFVWNTdqMHJMLUR0OU1Ja1FUWmxXam5kcVVOZG1Ecnlyd0E1OU8zS1lWYWY5cmUwdW9QS3ZGUmRmY3cifQ".to_string(),
        }
    }
}

impl BasicIssuerConfigTrait for BasicIssuerConfig {
    fn get_host_without_protocol(&self) -> String {
        let host = self.host.clone();
        match host.port {
            Some(port) => {
                format!("{}:{}", host.url, port)
            }
            None => {
                format!("{}", host.url,)
            }
        }
    }

    fn get_host(&self) -> String {
        let host = self.host.clone();
        match host.port {
            Some(port) => {
                format!("{}://{}:{}", host.protocol, host.url, port)
            }
            None => {
                format!("{}://{}", host.protocol, host.url)
            }
        }
    }

    fn is_local(&self) -> bool {
        self.is_local
    }

    fn get_cert(&self) -> anyhow::Result<String> {
        let path = format!("{}/cert.pem", self.keys_path);
        read(&path)
    }
    fn get_priv_key(&self) -> anyhow::Result<String> {
        let path = format!("{}/private_key.pem", self.keys_path);
        read(&path)
    }
    fn get_pub_key(&self) -> anyhow::Result<String> {
        let path = format!("{}/public_key.pem", self.keys_path);
        read(&path)
    }
    fn get_api_path(&self) -> String {
        self.api_path.clone()
    }
    fn get_w3c_data_model(&self) -> Option<W3cDataModelVersion> {
        self.w3c_vc_data_model.clone()
    }
    fn get_did(&self) -> String {
        self.did.clone()
    }

}
