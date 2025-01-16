use axum::async_trait;
mod macros;
mod rejections;
use rejections::*;

use axum::extract::FromRequestParts;
use axum_core::body::Body;
use axum_core::response::Response;
use axum_core::Error;
use bytes::Bytes;
use futures_util::{
    sink::{Sink, SinkExt},
    stream::{Stream, StreamExt},
};
use http::{
    header::{self, HeaderMap, HeaderName, HeaderValue},
    request::Parts,
    Method, StatusCode, Version,
};
use hyper_util::rt::TokioIo;
use sha1::{Digest, Sha1};
use std::{
    borrow::Cow,
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};
use tokio_tungstenite::{
    tungstenite::{
        self as ts,
        protocol::{self, WebSocketConfig},
    },
    WebSocketStream,
};

pub struct WebSocketUpgrade<F = DefaultOnFailedUpgrade> {
    config: WebSocketConfig,
    protocol: Option<HeaderValue>,
    sec_websocket_key: Option<HeaderValue>,
    on_upgrade: hyper::upgrade::OnUpgrade,
    on_failed_upgrade: F,
    sec_websocket_protocol: Option<HeaderValue>,
}

impl<F> std::fmt::Debug for WebSocketUpgrade<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebSocketUpgrade")
            .field("config", &self.config)
            .field("protocol", &self.protocol)
            .field("sec_websocket_key", &self.sec_websocket_key)
            .field("sec_websocket_protocol", &self.sec_websocket_protocol)
            .finish_non_exhaustive()
    }
}

impl<F> WebSocketUpgrade<F> {
    pub fn read_buffer_size(mut self, size: usize) -> Self {
        self.config.read_buffer_size = size;
        self
    }

    pub fn write_buffer_size(mut self, size: usize) -> Self {
        self.config.write_buffer_size = size;
        self
    }

    pub fn max_write_buffer_size(mut self, max: usize) -> Self {
        self.config.max_write_buffer_size = max;
        self
    }

    /// Set the maximum message size (defaults to 64 megabytes)
    pub fn max_message_size(mut self, max: usize) -> Self {
        self.config.max_message_size = Some(max);
        self
    }

    /// Set the maximum frame size (defaults to 16 megabytes)
    pub fn max_frame_size(mut self, max: usize) -> Self {
        self.config.max_frame_size = Some(max);
        self
    }

    /// Allow server to accept unmasked frames (defaults to false)
    pub fn accept_unmasked_frames(mut self, accept: bool) -> Self {
        self.config.accept_unmasked_frames = accept;
        self
    }

    pub fn protocols<I>(mut self, protocols: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Cow<'static, str>>,
    {
        if let Some(req_protocols) = self
            .sec_websocket_protocol
            .as_ref()
            .and_then(|p| p.to_str().ok())
        {
            self.protocol = protocols
                .into_iter()
                // FIXME: This will often allocate a new `String` and so is less efficient than it
                // could be. But that can't be fixed without breaking changes to the public API.
                .map(Into::into)
                .find(|protocol| {
                    req_protocols
                        .split(',')
                        .any(|req_protocol| req_protocol.trim() == protocol)
                })
                .map(|protocol| match protocol {
                    Cow::Owned(s) => HeaderValue::from_str(&s).unwrap(),
                    Cow::Borrowed(s) => HeaderValue::from_static(s),
                });
        }

        self
    }

    pub fn on_failed_upgrade<C>(self, callback: C) -> WebSocketUpgrade<C>
    where
        C: OnFailedUpgrade,
    {
        WebSocketUpgrade {
            config: self.config,
            protocol: self.protocol,
            sec_websocket_key: self.sec_websocket_key,
            on_upgrade: self.on_upgrade,
            on_failed_upgrade: callback,
            sec_websocket_protocol: self.sec_websocket_protocol,
        }
    }

    /// Finalize upgrading the connection and call the provided callback with
    /// the stream.
    #[must_use = "to set up the WebSocket connection, this response must be returned"]
    pub fn on_upgrade<C, Fut>(self, callback: C) -> Response
    where
        C: FnOnce(WebSocket) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
        F: OnFailedUpgrade,
    {
        let on_upgrade = self.on_upgrade;
        let config = self.config;
        let on_failed_upgrade = self.on_failed_upgrade;

        let protocol = self.protocol.clone();

        tokio::spawn(async move {
            let upgraded = match on_upgrade.await {
                Ok(upgraded) => upgraded,
                Err(err) => {
                    on_failed_upgrade.call(Error::new(err));
                    return;
                }
            };
            let upgraded = TokioIo::new(upgraded);

            let socket =
                WebSocketStream::from_raw_socket(upgraded, protocol::Role::Server, Some(config))
                    .await;
            let socket = WebSocket {
                inner: socket,
                protocol,
            };
            callback(socket).await;
        });

        if let Some(sec_websocket_key) = &self.sec_websocket_key {
            // If `sec_websocket_key` was `Some`, we are using HTTP/1.1.

            #[allow(clippy::declare_interior_mutable_const)]
            const UPGRADE: HeaderValue = HeaderValue::from_static("upgrade");
            #[allow(clippy::declare_interior_mutable_const)]
            const WEBSOCKET: HeaderValue = HeaderValue::from_static("websocket");

            let mut builder = Response::builder()
                .status(StatusCode::SWITCHING_PROTOCOLS)
                .header(header::CONNECTION, UPGRADE)
                .header(header::UPGRADE, WEBSOCKET)
                .header(
                    header::SEC_WEBSOCKET_ACCEPT,
                    sign(sec_websocket_key.as_bytes()),
                );

            if let Some(protocol) = self.protocol {
                builder = builder.header(header::SEC_WEBSOCKET_PROTOCOL, protocol);
            }

            builder.body(Body::empty()).unwrap()
        } else {
            // Otherwise, we are HTTP/2+. As established in RFC 9113 section 8.5, we just respond
            // with a 2XX with an empty body:
            // <https://datatracker.ietf.org/doc/html/rfc9113#name-the-connect-method>.
            Response::new(Body::empty())
        }
    }
}

/// What to do when a connection upgrade fails.
///
/// See [`WebSocketUpgrade::on_failed_upgrade`] for more details.
pub trait OnFailedUpgrade: Send + 'static {
    /// Call the callback.
    fn call(self, error: Error);
}

impl<F> OnFailedUpgrade for F
where
    F: FnOnce(Error) + Send + 'static,
{
    fn call(self, error: Error) {
        self(error)
    }
}

/// The default `OnFailedUpgrade` used by `WebSocketUpgrade`.
///
/// It simply ignores the error.
#[non_exhaustive]
#[derive(Debug)]
pub struct DefaultOnFailedUpgrade;

impl OnFailedUpgrade for DefaultOnFailedUpgrade {
    #[inline]
    fn call(self, _error: Error) {}
}

#[async_trait]
impl<S> FromRequestParts<S> for WebSocketUpgrade<DefaultOnFailedUpgrade>
where
    S: Send + Sync,
{
    type Rejection = WebSocketUpgradeRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let sec_websocket_key = if parts.version <= Version::HTTP_11 {
            if parts.method != Method::GET {
                return Err(MethodNotGet.into());
            }

            if !header_contains(&parts.headers, header::CONNECTION, "upgrade") {
                return Err(InvalidConnectionHeader.into());
            }

            if !header_eq(&parts.headers, header::UPGRADE, "websocket") {
                return Err(InvalidUpgradeHeader.into());
            }

            Some(
                parts
                    .headers
                    .get(header::SEC_WEBSOCKET_KEY)
                    .ok_or(WebSocketKeyHeaderMissing)?
                    .clone(),
            )
        } else {
            if parts.method != Method::CONNECT {
                return Err(MethodNotConnect.into());
            }

            // if this feature flag is disabled, we won’t be receiving an HTTP/2 request to begin
            // with.
            #[cfg(feature = "http2")]
            if parts
                .extensions
                .get::<hyper::ext::Protocol>()
                .map_or(true, |p| p.as_str() != "websocket")
            {
                return Err(InvalidProtocolPseudoheader.into());
            }

            None
        };

        if !header_eq(&parts.headers, header::SEC_WEBSOCKET_VERSION, "13") {
            return Err(InvalidWebSocketVersionHeader.into());
        }

        let on_upgrade = parts
            .extensions
            .remove::<hyper::upgrade::OnUpgrade>()
            .ok_or(ConnectionNotUpgradable)?;

        let sec_websocket_protocol = parts.headers.get(header::SEC_WEBSOCKET_PROTOCOL).cloned();

        Ok(Self {
            config: Default::default(),
            protocol: None,
            sec_websocket_key,
            on_upgrade,
            sec_websocket_protocol,
            on_failed_upgrade: DefaultOnFailedUpgrade,
        })
    }
}

fn header_eq(headers: &HeaderMap, key: HeaderName, value: &'static str) -> bool {
    if let Some(header) = headers.get(&key) {
        header.as_bytes().eq_ignore_ascii_case(value.as_bytes())
    } else {
        false
    }
}

fn header_contains(headers: &HeaderMap, key: HeaderName, value: &'static str) -> bool {
    let header = if let Some(header) = headers.get(&key) {
        header
    } else {
        return false;
    };

    if let Ok(header) = std::str::from_utf8(header.as_bytes()) {
        header.to_ascii_lowercase().contains(value)
    } else {
        false
    }
}

/// A stream of WebSocket messages.
///
/// See [the module level documentation](self) for more details.
#[derive(Debug)]
pub struct WebSocket {
    inner: WebSocketStream<TokioIo<hyper::upgrade::Upgraded>>,
    protocol: Option<HeaderValue>,
}

impl WebSocket {
    /// Receive another message.
    ///
    /// Returns `None` if the stream has closed.
    pub async fn recv(&mut self) -> Option<Result<Message, Error>> {
        self.next().await
    }

    /// Send a message.
    pub async fn send(&mut self, msg: Message) -> Result<(), Error> {
        self.inner
            .send(msg.into_tungstenite())
            .await
            .map_err(Error::new)
    }

    /// Return the selected WebSocket subprotocol, if one has been chosen.
    pub fn protocol(&self) -> Option<&HeaderValue> {
        self.protocol.as_ref()
    }
}

impl Stream for WebSocket {
    type Item = Result<Message, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match ready!(self.inner.poll_next_unpin(cx)) {
                Some(Ok(msg)) => {
                    if let Some(msg) = Message::from_tungstenite(msg) {
                        return Poll::Ready(Some(Ok(msg)));
                    }
                }
                Some(Err(err)) => return Poll::Ready(Some(Err(Error::new(err)))),
                None => return Poll::Ready(None),
            }
        }
    }
}

impl Sink<Message> for WebSocket {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_ready(cx).map_err(Error::new)
    }

    fn start_send(mut self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        Pin::new(&mut self.inner)
            .start_send(item.into_tungstenite())
            .map_err(Error::new)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_flush(cx).map_err(Error::new)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_close(cx).map_err(Error::new)
    }
}

/// UTF-8 wrapper for [Bytes].
///
/// An [Utf8Bytes] is always guaranteed to contain valid UTF-8.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Utf8Bytes(ts::Utf8Bytes);

impl Utf8Bytes {
    /// Creates from a static str.
    #[inline]
    pub const fn from_static(str: &'static str) -> Self {
        Self(ts::Utf8Bytes::from_static(str))
    }

    /// Returns as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn into_tungstenite(self) -> ts::Utf8Bytes {
        self.0
    }
}

impl std::ops::Deref for Utf8Bytes {
    type Target = str;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl std::fmt::Display for Utf8Bytes {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<Bytes> for Utf8Bytes {
    type Error = std::str::Utf8Error;

    #[inline]
    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(bytes.try_into()?))
    }
}

impl TryFrom<Vec<u8>> for Utf8Bytes {
    type Error = std::str::Utf8Error;

    #[inline]
    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(v.try_into()?))
    }
}

impl From<String> for Utf8Bytes {
    #[inline]
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl From<&str> for Utf8Bytes {
    #[inline]
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl From<&String> for Utf8Bytes {
    #[inline]
    fn from(s: &String) -> Self {
        Self(s.into())
    }
}

impl From<Utf8Bytes> for Bytes {
    #[inline]
    fn from(Utf8Bytes(bytes): Utf8Bytes) -> Self {
        bytes.into()
    }
}

impl<T> PartialEq<T> for Utf8Bytes
where
    for<'a> &'a str: PartialEq<T>,
{
    #[inline]
    fn eq(&self, other: &T) -> bool {
        self.as_str() == *other
    }
}

/// Status code used to indicate why an endpoint is closing the WebSocket connection.
pub type CloseCode = u16;

/// A struct representing the close command.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CloseFrame {
    /// The reason as a code.
    pub code: CloseCode,
    /// The reason as text string.
    pub reason: Utf8Bytes,
}

/// A WebSocket message.
//
// This code comes from https://github.com/snapview/tungstenite-rs/blob/master/src/protocol/message.rs and is under following license:
// Copyright (c) 2017 Alexey Galakhov
// Copyright (c) 2016 Jason Housley
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Message {
    /// A text WebSocket message
    Text(Utf8Bytes),
    /// A binary WebSocket message
    Binary(Bytes),
    /// A ping message with the specified payload
    ///
    /// The payload here must have a length less than 125 bytes.
    ///
    /// Ping messages will be automatically responded to by the server, so you do not have to worry
    /// about dealing with them yourself.
    Ping(Bytes),
    /// A pong message with the specified payload
    ///
    /// The payload here must have a length less than 125 bytes.
    ///
    /// Pong messages will be automatically sent to the client if a ping message is received, so
    /// you do not have to worry about constructing them yourself unless you want to implement a
    /// [unidirectional heartbeat](https://tools.ietf.org/html/rfc6455#section-5.5.3).
    Pong(Bytes),
    /// A close message with the optional close frame.
    ///
    /// You may "uncleanly" close a WebSocket connection at any time
    /// by simply dropping the [`WebSocket`].
    /// However, you may also use the graceful closing protocol, in which
    /// 1. peer A sends a close frame, and does not send any further messages;
    /// 2. peer B responds with a close frame, and does not send any further messages;
    /// 3. peer A processes the remaining messages sent by peer B, before finally
    /// 4. both peers close the connection.
    ///
    /// After sending a close frame,
    /// you may still read messages,
    /// but attempts to send another message will error.
    /// After receiving a close frame,
    /// axum will automatically respond with a close frame if necessary
    /// (you do not have to deal with this yourself).
    /// Since no further messages will be received,
    /// you may either do nothing
    /// or explicitly drop the connection.
    Close(Option<CloseFrame>),
}

impl Message {
    fn into_tungstenite(self) -> ts::Message {
        match self {
            Self::Text(text) => ts::Message::Text(text.into_tungstenite()),
            Self::Binary(binary) => ts::Message::Binary(binary),
            Self::Ping(ping) => ts::Message::Ping(ping),
            Self::Pong(pong) => ts::Message::Pong(pong),
            Self::Close(Some(close)) => ts::Message::Close(Some(ts::protocol::CloseFrame {
                code: ts::protocol::frame::coding::CloseCode::from(close.code),
                reason: close.reason.into_tungstenite(),
            })),
            Self::Close(None) => ts::Message::Close(None),
        }
    }

    fn from_tungstenite(message: ts::Message) -> Option<Self> {
        match message {
            ts::Message::Text(text) => Some(Self::Text(Utf8Bytes(text))),
            ts::Message::Binary(binary) => Some(Self::Binary(binary)),
            ts::Message::Ping(ping) => Some(Self::Ping(ping)),
            ts::Message::Pong(pong) => Some(Self::Pong(pong)),
            ts::Message::Close(Some(close)) => Some(Self::Close(Some(CloseFrame {
                code: close.code.into(),
                reason: Utf8Bytes(close.reason),
            }))),
            ts::Message::Close(None) => Some(Self::Close(None)),
            // we can ignore `Frame` frames as recommended by the tungstenite maintainers
            // https://github.com/snapview/tungstenite-rs/issues/268
            ts::Message::Frame(_) => None,
        }
    }

    /// Consume the WebSocket and return it as binary data.
    pub fn into_data(self) -> Bytes {
        match self {
            Self::Text(string) => Bytes::from(string),
            Self::Binary(data) | Self::Ping(data) | Self::Pong(data) => data,
            Self::Close(None) => Bytes::new(),
            Self::Close(Some(frame)) => Bytes::from(frame.reason),
        }
    }

    /// Attempt to consume the WebSocket message and convert it to a Utf8Bytes.
    pub fn into_text(self) -> Result<Utf8Bytes, Error> {
        match self {
            Self::Text(string) => Ok(string),
            Self::Binary(data) | Self::Ping(data) | Self::Pong(data) => {
                Ok(Utf8Bytes::try_from(data).map_err(Error::new)?)
            }
            Self::Close(None) => Ok(Utf8Bytes::default()),
            Self::Close(Some(frame)) => Ok(frame.reason),
        }
    }

    /// Attempt to get a &str from the WebSocket message,
    /// this will try to convert binary data to utf8.
    pub fn to_text(&self) -> Result<&str, Error> {
        match *self {
            Self::Text(ref string) => Ok(string.as_str()),
            Self::Binary(ref data) | Self::Ping(ref data) | Self::Pong(ref data) => {
                Ok(std::str::from_utf8(data).map_err(Error::new)?)
            }
            Self::Close(None) => Ok(""),
            Self::Close(Some(ref frame)) => Ok(&frame.reason),
        }
    }

    /// Create a new text WebSocket message from a stringable.
    pub fn text<S>(string: S) -> Message
    where
        S: Into<Utf8Bytes>,
    {
        Message::Text(string.into())
    }

    /// Create a new binary WebSocket message by converting to `Bytes`.
    pub fn binary<B>(bin: B) -> Message
    where
        B: Into<Bytes>,
    {
        Message::Binary(bin.into())
    }
}

impl From<String> for Message {
    fn from(string: String) -> Self {
        Message::Text(string.into())
    }
}

impl<'s> From<&'s str> for Message {
    fn from(string: &'s str) -> Self {
        Message::Text(string.into())
    }
}

impl<'b> From<&'b [u8]> for Message {
    fn from(data: &'b [u8]) -> Self {
        Message::Binary(Bytes::copy_from_slice(data))
    }
}

impl From<Vec<u8>> for Message {
    fn from(data: Vec<u8>) -> Self {
        Message::Binary(data.into())
    }
}

impl From<Message> for Vec<u8> {
    fn from(msg: Message) -> Self {
        msg.into_data().to_vec()
    }
}

fn sign(key: &[u8]) -> HeaderValue {
    use base64::engine::Engine as _;

    let mut sha1 = Sha1::default();
    sha1.update(key);
    sha1.update(&b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11"[..]);
    let b64 = Bytes::from(base64::engine::general_purpose::STANDARD.encode(sha1.finalize()));
    HeaderValue::from_maybe_shared(b64).expect("base64 is a valid value")
}
