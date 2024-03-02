use std::{fmt, ops::Deref};

use async_trait::async_trait;
use axum::{
    extract::FromRequest,
    http::{self, Request},
    response::{IntoResponse, Response},
};
use tracing::{Level, Span};
use uuid::Uuid;

/// An identifier for a request.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RequestId(Uuid);

/// Rejection type for `RequestId` if the identifier for a request was not found.
#[derive(Debug, thiserror::Error)]
#[error("missing request identifier")]
pub struct MissingRequestId;

impl IntoResponse for MissingRequestId {
    fn into_response(self) -> Response {
        http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

#[async_trait]
impl<STATE, BODY> FromRequest<STATE, BODY> for RequestId
where
    BODY: Send + 'static,
    STATE: Send + Sync,
{
    type Rejection = MissingRequestId;

    async fn from_request(req: Request<BODY>, _state: &STATE) -> Result<Self, Self::Rejection> {
        req.extensions()
            .get::<RequestId>()
            .cloned()
            .ok_or(MissingRequestId)
    }
}

impl Deref for RequestId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl RequestId {
    /// Create a new `RequestId`.
    #[inline]
    fn new() -> Self {
        RequestId(Uuid::new_v4())
    }
}

/// `Layer` for adding an identifier for a request.
#[derive(Clone, Copy, Debug)]
pub struct AddRequestIdLayer;

impl<S> tower::Layer<S> for AddRequestIdLayer {
    type Service = AddRequestId<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AddRequestId { inner }
    }
}

/// Middleware for adding an identifier for a request.
#[derive(Clone, Copy, Debug)]
pub struct AddRequestId<S> {
    inner: S,
}

impl<B, S> tower::Service<Request<B>> for AddRequestId<S>
where
    S: tower::Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        req.extensions_mut().insert(RequestId::new());
        self.inner.call(req)
    }
}

/// Set the 'x-request-id' header value using the identifier for a request.
#[derive(Debug, Clone, Copy)]
pub struct UseRequestId;

impl tower_http::request_id::MakeRequestId for UseRequestId {
    fn make_request_id<B>(
        &mut self,
        req: &Request<B>,
    ) -> Option<tower_http::request_id::RequestId> {
        req.extensions().get::<RequestId>().map(|request_id| {
            http::HeaderValue::from_str(&request_id.to_string())
                .unwrap()
                .into()
        })
    }
}

/// Make span from a request using its identifier.
#[derive(Debug, Clone)]
pub struct MakeSpanWithRequestId {
    level: Level,
}

impl MakeSpanWithRequestId {
    pub fn new() -> Self {
        MakeSpanWithRequestId {
            level: Level::DEBUG,
        }
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
}

impl Default for MakeSpanWithRequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl<B> tower_http::trace::MakeSpan<B> for MakeSpanWithRequestId {
    fn make_span(&mut self, req: &Request<B>) -> Span {
        macro_rules! make_span {
            ($level:expr) => {
                if let Some(request_id) = req.extensions().get::<RequestId>() {
                    tracing::span!(
                        $level,
                        "request",
                        method = %req.method(),
                        uri = %req.uri(),
                        version = ?req.version(),
                        request_id = %request_id,
                    )
                } else {
                    tracing::span!(
                        $level,
                        "request",
                        method = %req.method(),
                        uri = %req.uri(),
                        version = ?req.version(),
                    )
                }
            }
        }

        match self.level {
            Level::ERROR => {
                make_span!(Level::ERROR)
            }
            Level::WARN => {
                make_span!(Level::WARN)
            }
            Level::INFO => {
                make_span!(Level::INFO)
            }
            Level::DEBUG => {
                make_span!(Level::DEBUG)
            }
            Level::TRACE => {
                make_span!(Level::TRACE)
            }
        }
    }
}
