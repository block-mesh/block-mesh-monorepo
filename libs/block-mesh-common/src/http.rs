#[cfg(feature = "http")]
use bytes::Bytes;
#[cfg(feature = "http")]
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
#[cfg(feature = "http")]
use hyper::http;

#[cfg(feature = "http")]
pub fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

#[cfg(feature = "http")]
pub fn host_addr(uri: &http::Uri) -> Option<String> {
    uri.authority().map(|auth| auth.to_string())
}

#[cfg(feature = "http")]
pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
