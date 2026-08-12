// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    cell::RefCell,
    io::{Cursor, Read, Write},
    rc::Rc,
};

use serde::Deserialize;

use super::GmpClient;
use crate::commands::version::{GetVersionRequest, GetVersionResponse};

struct TestSocket {
    read_data: Cursor<Vec<u8>>,
    write_buffer: Rc<RefCell<Vec<u8>>>,
}

impl Read for TestSocket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.read_data.read(buf)
    }
}

impl Write for TestSocket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_buffer.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "authenticate_response")]
struct AuthenticateResponseTest {
    #[serde(rename = "@status")]
    status: u16,
    role: String,
}

#[test]
fn receive_reads_until_first_root_element_is_closed() {
    let payload =
        b"<authenticate_response status='200'><role>Admin</role></authenticate_response><next/>"
            .to_vec();
    let mut client = GmpClient::new(Cursor::new(payload));

    let response = client.receive().expect("failed to receive response");

    assert_eq!(
        response,
        "<authenticate_response status='200'><role>Admin</role></authenticate_response>"
    );
}

#[test]
fn receive_stops_after_first_empty_root_element() {
    let payload = b"<first/><second/>".to_vec();
    let mut client = GmpClient::new(Cursor::new(payload));

    let response = client.receive().expect("failed to receive response");

    assert_eq!(response, "<first/>");
}

#[test]
fn receive_reads_two_consecutive_root_elements() {
    let payload = b"<first><id>1</id></first><second><id>2</id></second>".to_vec();
    let mut client = GmpClient::new(Cursor::new(payload));

    let first = client.receive().expect("failed to receive first response");
    let second = client.receive().expect("failed to receive second response");

    assert_eq!(first, "<first><id>1</id></first>");
    assert_eq!(second, "<second><id>2</id></second>");
}

#[test]
fn receive_response_deserializes_to_typed_struct() {
    let payload =
        b"<authenticate_response status='200' status_text='OK'><role>Admin</role></authenticate_response>".to_vec();
    let mut client = GmpClient::new(Cursor::new(payload));

    let response: AuthenticateResponseTest = client
        .receive_response()
        .expect("failed to receive typed response");

    assert_eq!(response.status, 200);
    assert_eq!(response.role, "Admin");
}

#[test]
fn send_command_serializes_and_writes_xml_to_socket() {
    let shared_buffer = Rc::new(RefCell::new(Vec::new()));
    let socket = TestSocket {
        read_data: Cursor::new(
            b"<get_version_response status=\"200\" status_text=\"OK\"><version>22.4</version></get_version_response>"
                .to_vec(),
        ),
        write_buffer: Rc::clone(&shared_buffer),
    };
    let mut client = GmpClient::new(socket);

    client
        .send_command::<_, GetVersionResponse>(&GetVersionRequest)
        .expect("failed to send command");

    let written = String::from_utf8(shared_buffer.borrow().clone()).expect("invalid utf-8 written");

    assert_eq!(written, "<get_version/>");
}

#[test]
fn receive_response_returns_deserialize_error_on_premature_eof() {
    let payload = b"<authenticate_response status='200'><role>Admin</role>".to_vec();
    let mut client = GmpClient::new(Cursor::new(payload));

    let result: Result<AuthenticateResponseTest, crate::errors::Error> = client.receive_response();

    assert!(
        matches!(result, Err(crate::errors::Error::DeserializeError(_))),
        "expected deserialize error for premature EOF, got: {result:?}"
    );
}

#[test]
fn returns_gmp_response_error() {
    let payload = b"<authenticate_response status='400' status_text='Bad Request'/>".to_vec();
    let mut client = GmpClient::new(Cursor::new(payload));

    let result: Result<AuthenticateResponseTest, crate::errors::Error> = client.receive_response();

    assert!(
        matches!(result, Err(crate::errors::Error::GmpResponseError { response }) if response.status == 400 && response.status_text == "Bad Request"),
        "expected GMP response error for status 400"
    );
}
