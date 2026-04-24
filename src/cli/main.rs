// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use gvm_rs::{
    commands::{
        authenticate::{AuthenticateRequest, AuthenticateResponse},
        target::{GetTargetsRequest, GetTargetsResponse},
        version::{GetVersionRequest, GetVersionResponse},
    },
    connection::GmpConnection,
};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("GVM_LOG"))
        .init();

    let mut connection =
        gvm_rs::connection::UnixSocketConnection::new("/tmp/gvm/gvmd/gvmd.sock").unwrap();
    let version_request = GetVersionRequest::default();

    connection.send_command(&version_request).unwrap();
    let version_response: GetVersionResponse = connection.receive_response().unwrap();

    println!("Received response: {:?}", version_response);

    let auth_request = AuthenticateRequest::new("admin", "admin", true);
    connection.send_command(&auth_request).unwrap();
    let auth_response: AuthenticateResponse = connection.receive_response().unwrap();
    println!("Authentication response: {:?}", auth_response);

    let get_targets_request = GetTargetsRequest::new(Some(true), None, None, None);
    connection.send_command(&get_targets_request).unwrap();
    let targets_response = connection.receive_response::<GetTargetsResponse>().unwrap();
    println!("Targets response: {:?}", targets_response);
}
