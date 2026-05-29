// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use gvm_rs::{
    client::GmpClient,
    commands::{
        authenticate::{AuthenticateRequest, AuthenticateResponse},
        target::{GetTargetsRequest, GetTargetsResponse},
        version::{GetVersionRequest, GetVersionResponse},
    },
};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("GVM_LOG"))
        .init();

    let mut client = GmpClient::from_unix_socket_path("/tmp/gvm/gvmd/gvmd.sock").unwrap();
    let version_request = GetVersionRequest;

    client.send_command(&version_request).unwrap();
    let version_response: GetVersionResponse = client.receive_response().unwrap();

    println!("Received response: {:?}", version_response);

    let auth_request = AuthenticateRequest::new("admin", "admin").with_token();
    client.send_command(&auth_request).unwrap();
    let auth_response: AuthenticateResponse = client.receive_response().unwrap();
    println!("Authentication response: {:?}", auth_response);

    let get_targets_request = GetTargetsRequest::new().with_tasks();
    client.send_command(&get_targets_request).unwrap();
    let targets_response = client.receive_response::<GetTargetsResponse>().unwrap();
    println!("Targets response: {:?}", targets_response);
}
