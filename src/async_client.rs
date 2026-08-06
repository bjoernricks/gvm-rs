// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::Path;

use quick_xml::{Reader, Writer, events::Event};
use serde::{Serialize, de::DeserializeOwned};
use tokio::{
    io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    net::UnixStream,
};

pub struct GmpAsyncClient<T> {
    socket: T,
}

impl<T> GmpAsyncClient<T> {
    pub fn new(socket: T) -> Self {
        GmpAsyncClient { socket }
    }
}

impl<T: AsyncRead + Unpin> GmpAsyncClient<T> {
    async fn read_first_xml_element(&mut self) -> Result<String, crate::errors::Error> {
        let mut reader = Reader::from_reader(BufReader::new(&mut self.socket));
        let mut writer = Writer::new(Vec::new());
        let mut buf = Vec::new();
        let mut root_name: Option<String> = None;

        loop {
            match reader
                .read_event_into_async(&mut buf)
                .await
                .map_err(crate::errors::Error::XmlError)?
            {
                Event::Start(event) => {
                    if root_name.is_none() {
                        root_name =
                            Some(String::from_utf8_lossy(event.name().as_ref()).into_owned());
                    }

                    writer
                        .write_event(Event::Start(event.into_owned()))
                        .map_err(crate::errors::Error::ConnectionError)?;
                }
                Event::Empty(event) => {
                    if root_name.is_none() {
                        writer
                            .write_event(Event::Empty(event.into_owned()))
                            .map_err(crate::errors::Error::ConnectionError)?;
                        break;
                    }

                    writer
                        .write_event(Event::Empty(event.into_owned()))
                        .map_err(crate::errors::Error::ConnectionError)?;
                }
                Event::End(event) => {
                    if let Some(root_name) = root_name.as_ref() {
                        let is_root_end = root_name.as_bytes() == event.name().as_ref();
                        writer
                            .write_event(Event::End(event.into_owned()))
                            .map_err(crate::errors::Error::ConnectionError)?;
                        if is_root_end {
                            break;
                        }
                    }
                }
                Event::Eof => break,
                event => {
                    if root_name.is_some() {
                        writer
                            .write_event(event.into_owned())
                            .map_err(crate::errors::Error::ConnectionError)?;
                    }
                }
            }

            buf.clear();
        }

        let data_read = String::from_utf8_lossy(&writer.into_inner()).to_string();
        Ok(data_read)
    }

    async fn receive(&mut self) -> Result<String, crate::errors::Error> {
        let data_read = self.read_first_xml_element().await?;
        tracing::debug!("Received raw data: {}", data_read);
        Ok(data_read)
    }

    async fn receive_response<D>(&mut self) -> Result<D, crate::errors::Error>
    where
        D: DeserializeOwned,
    {
        let raw_response = self.receive().await?;
        let response = quick_xml::de::from_str(&raw_response)
            .map_err(crate::errors::Error::DeserializeError)?;
        Ok(response)
    }
}

impl<T: AsyncWrite + Unpin> GmpAsyncClient<T> {
    async fn send(&mut self, command: &str) -> Result<(), crate::errors::Error> {
        tracing::debug!("Sending command: {}", command);
        self.socket
            .write_all(command.as_bytes())
            .await
            .map_err(crate::errors::Error::ConnectionError)
    }
}

impl<T: AsyncRead + AsyncWrite + Unpin> GmpAsyncClient<T> {
    pub async fn send_command<S, D>(&mut self, command: &S) -> Result<D, crate::errors::Error>
    where
        S: Serialize,
        D: DeserializeOwned,
    {
        let command_str =
            quick_xml::se::to_string(command).map_err(crate::errors::Error::SerializeError)?;
        self.send(&command_str).await?;
        self.receive_response().await
    }
}

impl GmpAsyncClient<UnixStream> {
    pub async fn from_unix_socket_path<P: AsRef<Path>>(
        socket_path: P,
    ) -> Result<Self, crate::errors::Error> {
        match UnixStream::connect(socket_path).await {
            Ok(socket) => Ok(GmpAsyncClient::new(socket)),
            Err(error) => Err(crate::errors::Error::ConnectionError(error)),
        }
    }
}

#[cfg(test)]
#[path = "async_client_test.rs"]
mod tests;
