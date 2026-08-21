// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::CliOptions;
use gvm_rs::{
    async_client::GmpAsyncClient,
    commands::{
        authenticate::{AuthenticateRequest, AuthenticateResponse},
        target::{GetTargetsRequest, GetTargetsResponse},
        version::{GetVersionRequest, GetVersionResponse},
    },
    ssh::SshConfig,
    unix::UnixSocketConfig,
};
use std::time::Duration;

pub async fn send_commands(
    mut client: GmpAsyncClient<impl tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin>,
    options: &CliOptions,
) {
    let version_response: GetVersionResponse =
        client.send_command(&GetVersionRequest).await.unwrap();
    println!("Received response: {:?}", version_response);

    let auth_response: AuthenticateResponse = client
        .send_command(&AuthenticateRequest::new(
            &options.gmp_username,
            &options.gmp_password,
        ))
        .await
        .unwrap();
    println!("Authentication response: {:?}", auth_response);

    let targets_response: GetTargetsResponse = client
        .send_command(&GetTargetsRequest::new().with_tasks())
        .await
        .unwrap();
    println!("Targets response: {:?}", targets_response);
}

pub async fn run(options: &CliOptions) {
    match &options.command {
        crate::Commands::Socket(socket_command) => {
            let mut unix_config = UnixSocketConfig::new(&socket_command.socket_path);
            if let Some(timeout) = options.timeout {
                unix_config = unix_config.with_timeout(Duration::from_secs(timeout));
            }
            let client = GmpAsyncClient::from_unix_socket_config(&unix_config)
                .await
                .unwrap();
            send_commands(client, options).await;
        }
        crate::Commands::Ssh(ssh_command) => {
            let mut ssh_config = SshConfig::new(
                &ssh_command.ssh_hostname,
                ssh_command.ssh_port,
                &ssh_command.ssh_username,
            )
            .with_default_known_hosts_file();

            if ssh_command.ssh_auto_accept_host {
                ssh_config = ssh_config.with_auto_accept_host(true);
            }

            if let Some(timeout) = options.timeout {
                ssh_config = ssh_config.with_timeout(Duration::from_secs(timeout));
            }

            if let Some(password) = &ssh_command.ssh_password {
                ssh_config = ssh_config.with_password(password.clone());
            }

            let client = GmpAsyncClient::from_ssh_config(&ssh_config).await.unwrap();
            send_commands(client, options).await;
        }
    };
}
