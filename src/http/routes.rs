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

//! HTTP paths of the authority, shared by its routers and by the services that build URLs to
//! them, so both sides cannot drift apart.
//!
//! Every path goes after the API version (`get_api_version()`), except the `/.well-known`
//! documents, health and the console, which live at the root of the host. Paths with a
//! parameter are axum templates; to build a URL, fill them with [`ymir::http::routes::fill`].
//! Routes of ymir's own routers (wallet, verifier, health, OpenAPI) live in
//! [`ymir::http::routes`].

/// The authority's wallet; its routes are ymir's ([`ymir::http::routes::wallet`]).
pub mod wallet {
    pub const PREFIX: &str = "/wallet";
}

/// Session of the console.
pub mod session {
    /// Who the signed-in user is.
    pub const ME: &str = "/me";
}

/// Participants the authority has issued credentials to (minions).
pub mod minions {
    pub const PREFIX: &str = "/minions";
    pub const ALL: &str = "/all";
    pub const MYSELF: &str = "/myself";
    pub const BY_ID: &str = "/{id}";
}

/// Credential requests received, for an operator to approve or reject.
pub mod approver {
    pub const PREFIX: &str = "/approver";
    pub const ALL: &str = "/all";
    pub const BY_ID: &str = "/{id}";
    pub const DETAILS: &str = "/{id}/details";
}

/// GNAP gatekeeper (credential requests of the agents). Its access endpoint is the credential
/// issuer advertised in the authority's DID document.
pub mod gate {
    pub const PREFIX: &str = "/gate";
    /// Grant request of an agent.
    pub const ACCESS: &str = "/access";
    /// Continuation of a grant, by its continuation id.
    pub const CONTINUE: &str = "/continue/{id}";
}

/// OID4VCI issuer. ymir's issuer service builds the credential offer URI on its own
/// (`/credentialOffer`), so [`CREDENTIAL_OFFER`](issuer::CREDENTIAL_OFFER) has to match it.
pub mod issuer {
    pub const PREFIX: &str = "/issuer";
    pub const CREDENTIAL_OFFER: &str = "/credentialOffer";
    pub const TOKEN: &str = "/token";
    pub const CREDENTIAL: &str = "/credential";
    /// Issuer metadata, at the root of the host.
    pub const METADATA: &str = "/.well-known/openid-credential-issuer";
    /// Metadata of the issuer's authorization server, at the root of the host.
    pub const AS_METADATA: &str = "/.well-known/oauth-authorization-server";
}

/// Federated catalog: the participants of the data space, at the root of the host, for the
/// agents to import.
pub mod fed_catalog {
    pub const CATALOG: &str = "/.well-known/federated-catalog";
}

/// Notifications to the console.
pub mod react {
    pub const PREFIX: &str = "/react";
    /// Server-sent events stream.
    pub const NOTIFICATIONS_STREAM: &str = "/notifications/stream";
}

/// API documentation; its routes are ymir's ([`ymir::http::routes::openapi`]).
pub mod docs {
    pub const PREFIX: &str = "/docs";
}

/// Admin console (the React build), at the root of the host.
pub mod console {
    /// The host's root, which redirects to the console.
    pub const ROOT: &str = "/";
    pub const PREFIX: &str = "/admin";
}
