// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::tcp_connect;
use std::net::TcpListener;
use std::time::Duration;

#[test]
fn tcp_connect_applies_timeout_to_connected_socket() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind test listener");
    let address = listener
        .local_addr()
        .expect("failed to get listener address");

    let stream = tcp_connect("127.0.0.1", address.port(), Some(Duration::from_secs(1)))
        .expect("failed to connect to test listener");

    assert_eq!(
        stream.read_timeout().expect("failed to read timeout"),
        Some(Duration::from_secs(1))
    );
    assert_eq!(
        stream.write_timeout().expect("failed to read timeout"),
        Some(Duration::from_secs(1))
    );
}
