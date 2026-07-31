// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    cell::RefCell,
    io::{Cursor, Write},
    rc::Rc,
};

use serde::Deserialize;

use super::GmpClient;
use crate::commands::version::GetVersionRequest;

struct TestWriter {
    buffer: Rc<RefCell<Vec<u8>>>,
}

impl Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.borrow_mut().extend_from_slice(buf);
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
fn receive_response_deserializes_to_typed_struct() {
    let payload =
        b"<authenticate_response status='200'><role>Admin</role></authenticate_response>".to_vec();
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
    let writer = TestWriter {
        buffer: Rc::clone(&shared_buffer),
    };
    let mut client = GmpClient::new(writer);

    client
        .send_command(&GetVersionRequest)
        .expect("failed to send command");

    let written = String::from_utf8(shared_buffer.borrow().clone()).expect("invalid utf-8 written");

    assert_eq!(written, "<get_version/>");
}
