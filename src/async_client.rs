// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::deserialize::Response;
use quick_xml::{Reader, Writer, events::Event};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    future::Future,
    io,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, ReadBuf},
    net::UnixStream,
    time::{Sleep, sleep},
};

pub struct GmpAsyncClient<T> {
    stream: BufReader<TimeoutStream<T>>,
}

struct TimeoutStream<T> {
    stream: T,
    timeout: Option<Duration>,
    read_timeout: Option<Pin<Box<Sleep>>>,
    write_timeout: Option<Pin<Box<Sleep>>>,
    flush_timeout: Option<Pin<Box<Sleep>>>,
    shutdown_timeout: Option<Pin<Box<Sleep>>>,
}

impl<T> TimeoutStream<T> {
    fn new(stream: T) -> Self {
        Self {
            stream,
            timeout: None,
            read_timeout: None,
            write_timeout: None,
            flush_timeout: None,
            shutdown_timeout: None,
        }
    }

    fn poll_timeout(
        timeout: Option<Duration>,
        timer: &mut Option<Pin<Box<Sleep>>>,
        cx: &mut Context<'_>,
    ) -> bool {
        let Some(timeout) = timeout else {
            return false;
        };

        if timer.is_none() {
            *timer = Some(Box::pin(sleep(timeout)));
        }

        timer
            .as_mut()
            .is_some_and(|timer| timer.as_mut().poll(cx).is_ready())
    }

    fn timeout_error() -> io::Error {
        io::Error::new(io::ErrorKind::TimedOut, "GMP I/O operation timed out")
    }
}

impl<T: AsyncRead + Unpin> AsyncRead for TimeoutStream<T> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        if Self::poll_timeout(self.timeout, &mut self.read_timeout, cx) {
            self.read_timeout = None;
            return Poll::Ready(Err(Self::timeout_error()));
        }

        let result = Pin::new(&mut self.stream).poll_read(cx, buf);
        if result.is_ready() {
            self.read_timeout = None;
        }
        result
    }
}

impl<T: AsyncWrite + Unpin> AsyncWrite for TimeoutStream<T> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        if Self::poll_timeout(self.timeout, &mut self.write_timeout, cx) {
            self.write_timeout = None;
            return Poll::Ready(Err(Self::timeout_error()));
        }

        let result = Pin::new(&mut self.stream).poll_write(cx, buf);
        if result.is_ready() {
            self.write_timeout = None;
        }
        result
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        if Self::poll_timeout(self.timeout, &mut self.flush_timeout, cx) {
            self.flush_timeout = None;
            return Poll::Ready(Err(Self::timeout_error()));
        }

        let result = Pin::new(&mut self.stream).poll_flush(cx);
        if result.is_ready() {
            self.flush_timeout = None;
        }
        result
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        if Self::poll_timeout(self.timeout, &mut self.shutdown_timeout, cx) {
            self.shutdown_timeout = None;
            return Poll::Ready(Err(Self::timeout_error()));
        }

        let result = Pin::new(&mut self.stream).poll_shutdown(cx);
        if result.is_ready() {
            self.shutdown_timeout = None;
        }
        result
    }
}

impl<T: AsyncRead + Unpin> GmpAsyncClient<T> {
    pub fn new(socket: T) -> Self {
        GmpAsyncClient {
            stream: BufReader::new(TimeoutStream::new(socket)),
        }
    }

    pub fn with_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.stream.get_mut().timeout = timeout;
        self
    }

    async fn read_first_xml_element(&mut self) -> Result<String, crate::errors::Error> {
        let mut reader = Reader::from_reader(&mut self.stream);
        let mut writer = Writer::new(Vec::new());
        let mut buf = Vec::new();
        let mut root_name: Option<String> = None;

        loop {
            match reader
                .read_event_into_async(&mut buf)
                .await
                .map_err(crate::errors::Error::from_xml_error)?
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
        let status_response: Response = quick_xml::de::from_str(&raw_response)
            .map_err(crate::errors::Error::DeserializeError)?;

        if status_response.status >= 200 && status_response.status < 300 {
            let response = quick_xml::de::from_str(&raw_response)
                .map_err(crate::errors::Error::DeserializeError)?;
            Ok(response)
        } else {
            Err(crate::errors::Error::GmpResponseError {
                response: status_response,
            })
        }
    }
}

impl<T: AsyncRead + AsyncWrite + Unpin> GmpAsyncClient<T> {
    async fn send(&mut self, command: &str) -> Result<(), crate::errors::Error> {
        tracing::debug!("Sending command: {}", command);
        self.stream
            .get_mut()
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
        GmpAsyncClient::from_unix_socket_config(&crate::unix::UnixSocketConfig::new(socket_path))
            .await
    }

    pub async fn from_unix_socket_config(
        config: &crate::unix::UnixSocketConfig,
    ) -> Result<Self, crate::errors::Error> {
        match UnixStream::connect(&config.socket_path).await {
            Ok(socket) => Ok(GmpAsyncClient::new(socket).with_timeout(config.timeout)),
            Err(error) => Err(crate::errors::Error::ConnectionError(error)),
        }
    }
}

#[cfg(feature = "ssh-async")]
impl GmpAsyncClient<crate::ssh::SshAsyncStream> {
    pub async fn from_ssh_config(
        config: &crate::ssh::SshConfig,
    ) -> Result<Self, crate::errors::Error> {
        crate::ssh::connect_async(config)
            .await
            .map(|stream| GmpAsyncClient::new(stream).with_timeout(config.timeout))
    }
}

#[cfg(test)]
#[path = "async_client_test.rs"]
mod tests;
