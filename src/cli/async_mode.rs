// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use gvm_rs::{
    async_client::GmpAsyncClient,
    commands::{
        authenticate::{AuthenticateRequest, AuthenticateResponse},
        target::{GetTargetsRequest, GetTargetsResponse},
        version::{GetVersionRequest, GetVersionResponse},
    },
};

use crate::CliOptions;

pub async fn run(options: &CliOptions) {
    let mut client = GmpAsyncClient::from_unix_socket_path(&options.socket_path)
        .await
        .unwrap();

    let version_response: GetVersionResponse =
        client.send_command(&GetVersionRequest).await.unwrap();
    println!("Received response: {:?}", version_response);

    let auth_response: AuthenticateResponse = client
        .send_command(&AuthenticateRequest::new(&options.username, &options.password).with_token())
        .await
        .unwrap();
    println!("Authentication response: {:?}", auth_response);

    let targets_response: GetTargetsResponse = client
        .send_command(&GetTargetsRequest::new().with_tasks())
        .await
        .unwrap();
    println!("Targets response: {:?}", targets_response);
}
