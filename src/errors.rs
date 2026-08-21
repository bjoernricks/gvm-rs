// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::deserialize::Response;
use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An unknown error occurred")]
    UnknownError(),
    #[error("Could not connect: {0}")]
    ConnectionError(#[from] io::Error),
    #[error("Failed to parse response: {0}")]
    DeserializeError(#[from] quick_xml::DeError),
    #[error("Failed to serialize request: {0}")]
    SerializeError(#[from] quick_xml::SeError),
    #[error("XML error: {0}")]
    XmlError(#[from] quick_xml::Error),
    #[error("GMP response error: status {}, status_text: {}", response.status, response.status_text)]
    GmpResponseError { response: Response },
}

impl Error {
    pub(crate) fn from_xml_error(error: quick_xml::Error) -> Self {
        match error {
            quick_xml::Error::Io(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock
                ) =>
            {
                Self::ConnectionError(io::Error::new(io::ErrorKind::TimedOut, error.to_string()))
            }
            error => Self::XmlError(error),
        }
    }
}

#[cfg(test)]
#[path = "errors_test.rs"]
mod tests;
