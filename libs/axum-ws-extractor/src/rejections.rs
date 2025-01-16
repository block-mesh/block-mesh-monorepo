use crate::{composite_rejection, define_rejection};
use axum::response::IntoResponse;
use axum::response::Response;

define_rejection! {
    #[status = METHOD_NOT_ALLOWED]
    #[body = "Request method must be `GET`"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    pub struct MethodNotGet;
}

define_rejection! {
    #[status = METHOD_NOT_ALLOWED]
    #[body = "Request method must be `CONNECT`"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    pub struct MethodNotConnect;
}

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "Connection header did not include 'upgrade'"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    pub struct InvalidConnectionHeader;
}

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "`Upgrade` header did not include 'websocket'"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    pub struct InvalidUpgradeHeader;
}

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "`:protocol` pseudo-header did not include 'websocket'"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    pub struct InvalidProtocolPseudoheader;
}

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "`Sec-WebSocket-Version` header did not include '13'"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    pub struct InvalidWebSocketVersionHeader;
}

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "`Sec-WebSocket-Key` header missing"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    pub struct WebSocketKeyHeaderMissing;
}

define_rejection! {
    #[status = UPGRADE_REQUIRED]
    #[body = "WebSocket request couldn't be upgraded since no upgrade state was present"]
    /// Rejection type for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    ///
    /// This rejection is returned if the connection cannot be upgraded for example if the
    /// request is HTTP/1.0.
    ///
    /// See [MDN] for more details about connection upgrades.
    ///
    /// [MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Upgrade
    pub struct ConnectionNotUpgradable;
}

composite_rejection! {
    /// Rejection used for [`WebSocketUpgrade`](super::WebSocketUpgrade).
    ///
    /// Contains one variant for each way the [`WebSocketUpgrade`](super::WebSocketUpgrade)
    /// extractor can fail.
    pub enum WebSocketUpgradeRejection {
        MethodNotGet,
        MethodNotConnect,
        InvalidConnectionHeader,
        InvalidUpgradeHeader,
        InvalidProtocolPseudoheader,
        InvalidWebSocketVersionHeader,
        WebSocketKeyHeaderMissing,
        ConnectionNotUpgradable,
    }
}
