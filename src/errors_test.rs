// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;

use super::Error;

#[derive(Debug, Deserialize)]
#[serde(rename = "value")]
struct ValueWrapper {
    _value: String,
}

#[test]
fn unknown_error_has_expected_display_message() {
    let err = Error::UnknownError();

    assert_eq!(err.to_string(), "An unknown error occurred");
}

#[test]
fn from_io_error_creates_connection_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "connection refused");

    let err: Error = io_err.into();

    assert!(matches!(err, Error::ConnectionError(_)));
    assert!(err.to_string().contains("Could not connect:"));
}

#[test]
fn from_deserialize_error_creates_deserialize_error() {
    let de_err =
        quick_xml::de::from_str::<ValueWrapper>("<value>").expect_err("expected deserialize error");

    let err: Error = de_err.into();

    assert!(matches!(err, Error::DeserializeError(_)));
    assert!(err.to_string().contains("Failed to parse response:"));
}

#[test]
fn from_serialize_error_creates_serialize_error() {
    let se_err = quick_xml::se::to_string(&std::collections::HashMap::<String, String>::new())
        .expect_err("expected serialize error");

    let err: Error = se_err.into();

    assert!(matches!(err, Error::SerializeError(_)));
    assert!(err.to_string().contains("Failed to serialize request:"));
}

#[test]
fn gmp_response_error_has_expected_display_message() {
    let response = crate::deserialize::Response {
        status: 400,
        status_text: "Bad Request".to_string(),
    };
    let err = Error::GmpResponseError { response };

    assert_eq!(
        err.to_string(),
        "GMP response error: status 400, status_text: Bad Request"
    );
}
