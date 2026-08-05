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

    let version_response: GetVersionResponse = client.send_command(&GetVersionRequest).unwrap();
    println!("Received response: {:?}", version_response);

    let auth_response: AuthenticateResponse = client
        .send_command(&AuthenticateRequest::new("admin", "admin").with_token())
        .unwrap();
    println!("Authentication response: {:?}", auth_response);

    let targets_response: GetTargetsResponse = client
        .send_command(&GetTargetsRequest::new().with_tasks())
        .unwrap();
    println!("Targets response: {:?}", targets_response);
}
