// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

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
}
