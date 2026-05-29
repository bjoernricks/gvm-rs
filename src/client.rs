// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    io::{BufReader, Read, Write},
    os::unix::net::UnixStream,
    path::Path,
};

use quick_xml::{Reader, Writer, events::Event};
use serde::{Serialize, de::DeserializeOwned};

pub struct GmpClient<T> {
    socket: T,
}

impl<T> GmpClient<T> {
    pub fn new(socket: T) -> Self {
        GmpClient { socket }
    }
}

impl<T: Read> GmpClient<T> {
    fn read_first_xml_element(&mut self) -> Result<String, crate::errors::Error> {
        let mut reader = Reader::from_reader(BufReader::new(&mut self.socket));
        let mut writer = Writer::new(Vec::new());
        let mut buf = Vec::new();
        let mut root_name: Option<String> = None;

        loop {
            match reader
                .read_event_into(&mut buf)
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

    fn receive(&mut self) -> Result<String, crate::errors::Error> {
        let data_read = self.read_first_xml_element()?;
        tracing::debug!("Received raw data: {}", data_read);
        Ok(data_read)
    }

    pub fn receive_response<D>(&mut self) -> Result<D, crate::errors::Error>
    where
        D: DeserializeOwned,
    {
        let raw_response = self.receive()?;
        let response = quick_xml::de::from_str(&raw_response)
            .map_err(crate::errors::Error::DeserializeError)?;
        Ok(response)
    }
}

impl<T: Write> GmpClient<T> {
    fn send(&mut self, command: &str) -> Result<(), crate::errors::Error> {
        tracing::debug!("Sending command: {}", command);
        self.socket
            .write_all(command.as_bytes())
            .map_err(crate::errors::Error::ConnectionError)
    }

    pub fn send_command<S>(&mut self, command: &S) -> Result<(), crate::errors::Error>
    where
        S: Serialize,
    {
        let command_str =
            quick_xml::se::to_string(command).map_err(crate::errors::Error::SerializeError)?;
        self.send(&command_str)
    }
}

impl GmpClient<UnixStream> {
    pub fn from_unix_socket_path<P: AsRef<Path>>(
        socket_path: P,
    ) -> Result<Self, crate::errors::Error> {
        match UnixStream::connect(socket_path) {
            Ok(socket) => Ok(GmpClient::new(socket)),
            Err(e) => Err(crate::errors::Error::ConnectionError(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::GmpClient;

    use std::{io::Write, os::unix::net::UnixStream};

    #[test]
    fn receive_reads_until_first_root_element_is_closed() {
        let (mut writer, reader) = UnixStream::pair().expect("failed to create unix stream pair");
        let mut client = GmpClient::new(reader);

        writer
            .write_all(b"<authenticate_response status='200'><role>Admin</role></authenticate_response><next/>")
            .expect("failed to write test payload");

        let response = client.receive().expect("failed to receive response");

        assert_eq!(
            response,
            "<authenticate_response status='200'><role>Admin</role></authenticate_response>"
        );
    }
}
