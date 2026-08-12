// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::GmpAsyncClient;
use crate::commands::version::{GetVersionRequest, GetVersionResponse};

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "authenticate_response")]
struct AuthenticateResponseTest {
    #[serde(rename = "@status")]
    status: u16,
    role: String,
}

#[tokio::test(flavor = "current_thread")]
async fn receive_reads_until_first_root_element_is_closed() {
    let (client_stream, mut server_stream) = tokio::io::duplex(1024);
    let payload =
        b"<authenticate_response status='200'><role>Admin</role></authenticate_response><next/>";

    let writer = tokio::spawn(async move {
        server_stream
            .write_all(payload)
            .await
            .expect("failed to write response payload");
    });

    let mut client = GmpAsyncClient::new(client_stream);
    let response = client.receive().await.expect("failed to receive response");

    writer.await.expect("writer task failed");

    assert_eq!(
        response,
        "<authenticate_response status='200'><role>Admin</role></authenticate_response>"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn receive_stops_after_first_empty_root_element() {
    let (client_stream, mut server_stream) = tokio::io::duplex(128);

    let writer = tokio::spawn(async move {
        server_stream
            .write_all(b"<first/><second/>")
            .await
            .expect("failed to write response payload");
    });

    let mut client = GmpAsyncClient::new(client_stream);
    let response = client.receive().await.expect("failed to receive response");

    writer.await.expect("writer task failed");

    assert_eq!(response, "<first/>");
}

#[tokio::test(flavor = "current_thread")]
async fn receive_reads_two_consecutive_root_elements() {
    let (client_stream, mut server_stream) = tokio::io::duplex(512);
    let payload = b"<first><id>1</id></first><second><id>2</id></second>";

    let writer = tokio::spawn(async move {
        server_stream
            .write_all(payload)
            .await
            .expect("failed to write response payload");
    });

    let mut client = GmpAsyncClient::new(client_stream);
    let first = client
        .receive()
        .await
        .expect("failed to receive first response");
    let second = client
        .receive()
        .await
        .expect("failed to receive second response");

    writer.await.expect("writer task failed");

    assert_eq!(first, "<first><id>1</id></first>");
    assert_eq!(second, "<second><id>2</id></second>");
}

#[tokio::test(flavor = "current_thread")]
async fn receive_response_deserializes_to_typed_struct() {
    let (client_stream, mut server_stream) = tokio::io::duplex(512);
    let payload = b"<authenticate_response status='200' status_text='OK'><role>Admin</role></authenticate_response>";

    let writer = tokio::spawn(async move {
        server_stream
            .write_all(payload)
            .await
            .expect("failed to write response payload");
    });

    let mut client = GmpAsyncClient::new(client_stream);
    let response: AuthenticateResponseTest = client
        .receive_response()
        .await
        .expect("failed to receive typed response");

    writer.await.expect("writer task failed");

    assert_eq!(response.status, 200);
    assert_eq!(response.role, "Admin");
}

#[tokio::test(flavor = "current_thread")]
async fn send_command_serializes_and_writes_xml_to_socket() {
    let (client_stream, mut server_stream) = tokio::io::duplex(1024);

    let server = tokio::spawn(async move {
        let mut request = vec![0_u8; "<get_version/>".len()];
        server_stream
            .read_exact(&mut request)
            .await
            .expect("failed to read request payload");

        server_stream
            .write_all(
                b"<get_version_response status=\"200\" status_text=\"OK\"><version>22.4</version></get_version_response>",
            )
            .await
            .expect("failed to write response payload");

        String::from_utf8(request).expect("invalid utf-8 written")
    });

    let mut client = GmpAsyncClient::new(client_stream);

    client
        .send_command::<_, GetVersionResponse>(&GetVersionRequest)
        .await
        .expect("failed to send command");

    let written = server.await.expect("server task failed");

    assert_eq!(written, "<get_version/>");
}

#[tokio::test(flavor = "current_thread")]
async fn receive_response_returns_deserialize_error_on_premature_eof() {
    let (client_stream, mut server_stream) = tokio::io::duplex(512);
    let payload = b"<authenticate_response status='200'><role>Admin</role>";

    let writer = tokio::spawn(async move {
        server_stream
            .write_all(payload)
            .await
            .expect("failed to write response payload");
    });

    let mut client = GmpAsyncClient::new(client_stream);
    let result: Result<AuthenticateResponseTest, crate::errors::Error> =
        client.receive_response().await;

    writer.await.expect("writer task failed");

    assert!(
        matches!(result, Err(crate::errors::Error::DeserializeError(_))),
        "expected deserialize error for premature EOF, got: {result:?}"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn returns_gmp_response_error() {
    let (client_stream, mut server_stream) = tokio::io::duplex(512);
    let payload = b"<authenticate_response status='400' status_text='Bad Request'/>";

    let writer = tokio::spawn(async move {
        server_stream
            .write_all(payload)
            .await
            .expect("failed to write response payload");
    });

    let mut client = GmpAsyncClient::new(client_stream);
    let result: Result<AuthenticateResponseTest, crate::errors::Error> =
        client.receive_response().await;

    writer.await.expect("writer task failed");

    assert!(
        matches!(result, Err(crate::errors::Error::GmpResponseError { response }) if response.status == 400 && response.status_text == "Bad Request"),
        "expected GMP response error for status 400"
    );
}
