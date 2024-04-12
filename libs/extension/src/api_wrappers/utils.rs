use std::fmt::Debug;

// use std::time::Duration;
use crate::{constants::log, models::Payload, BrowserRuntime, Result, RetryAfter};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// Prepares and executes an HTTP request to spotify.
/// ## Types
/// * R - Response, always required
/// * P - Payload, may be omitted
/// ## Request types
/// * GET - if no payload is provided
/// * POST - if payload is provided
/// The requests includes token and other headers.
pub(super) async fn execute_http_request<R, P>(
    auth_header_value: &str,
    token_header_value: &str,
    url: &str,
    payload: Option<&Payload<P>>,
    runtime: &BrowserRuntime,
) -> Result<R>
where
    R: for<'de> serde::Deserialize<'de>,
    P: serde::Serialize,
{
    // log!("execute_get_request entered");
    // set request params
    let mut opts = RequestInit::new();
    opts.mode(RequestMode::Cors);
    match payload {
        Some(v) => {
            opts.method("POST");

            match serde_json::to_string(v) {
                Ok(v) => {
                    opts.body(Some(&wasm_bindgen::JsValue::from_str(&v)));
                }
                Err(e) => {
                    log!("Failed to serialize POST payload");
                    log!("{:?}", e);
                    // TODO: may be worth a retry
                    return Err(RetryAfter::Never);
                }
            }
        }
        None => {
            opts.method("GET");
        }
    }

    // log!("{url}");

    // create the request
    let request = match Request::new_with_str_and_init(url, &opts) {
        Ok(v) => v,
        Err(e) => {
            log!("Spotify request creation failed");
            log!("{:?}", e);
            // TODO: may be worth a retry
            return Err(RetryAfter::Never);
        }
    };

    // log!("Request created");

    // add headers
    let _ = request.headers().set("Accept", "application/json");
    // these keys were manually extracted from chrome webdev tools
    let _ = request.headers().set("authorization", auth_header_value);
    let _ = request.headers().set("client-token", token_header_value);
    if payload.is_some() {
        // only set the content type if there is POST payload
        let _ = request.headers().set("content-type", "application/json");
    }

    // log!("Headers set");

    // both window and globalscope have the same interface, but they are separate types so Rust has
    // to have separate paths for them
    // the output is the same type for both
    let resp_value = match runtime {
        BrowserRuntime::ChromeWorker(v) => JsFuture::from(v.fetch_with_request(&request)).await,
        BrowserRuntime::FireFoxWindow(v) => JsFuture::from(v.fetch_with_request(&request)).await,
    };

    // unwrap the response
    let resp_value = match resp_value {
        Ok(v) => v,
        Err(e) => {
            log!("Spotify request failed");
            log!("{url}");
            log!("{:?}", e);
            // TODO: may be worth a retry
            return Err(RetryAfter::Never);
        }
    };

    // exit if the response is not of the expected type
    if !resp_value.is_instance_of::<Response>() {
        log!("Spotify response in not Response");
        log!("{url}");
        log!("{:?}", resp_value);
        // TODO: may be worth a retry
        return Err(RetryAfter::Never);
    };

    // this is unlikely to fail because of the previous type check
    let resp: Response = match resp_value.dyn_into() {
        Ok(v) => v,
        Err(e) => {
            log!("Cannot typecast response to Response");
            log!("{url}");
            log!("{:?}", e);
            // TODO: may be worth a retry
            return Err(RetryAfter::Never);
        }
    };

    // Read the response stream to completion.
    // In theory, the stream may still be open and the op may take some time to complete
    let resp = match resp.json() {
        Ok(v) => JsFuture::from(v).await,
        Err(e) => {
            log!("Cannot convert Promise to Future");
            log!("{url}");
            log!("{:?}", e);
            // TODO: may be worth a retry
            return Err(RetryAfter::Never);
        }
    };

    // log!("HTTP request completed");

    // Unwrap the response and handle the error
    let resp = match resp {
        Ok(v) => v,
        Err(e) => {
            log!("Spotify request failed");
            log!("{url}");
            log!("{:?}", e);
            // TODO: may be worth a retry
            return Err(RetryAfter::Never);
        }
    };

    // log!("Resp as string:");
    // log!("{:?}", resp.as_string().unwrap());

    // convert into a rust struct
    let playlist = match serde_wasm_bindgen::from_value::<R>(resp) {
        Ok(v) => v,
        Err(e) => {
            log!("Cannot deser spotify response into rust struct");
            log!("{url}");
            log!("{:?}", e);
            return Err(RetryAfter::Never);
        }
    };

    Ok(playlist)
}

/// Returns `https://api-partner.spotify.com/pathfinder/v1/query`.
/// This could be a constant, but an fn more compatible with `build_get_url`.
pub(super) const BUILD_POST_URL: &str = "https://api-partner.spotify.com/pathfinder/v1/query";

/// Creates a URL with a query string for a GET request to Spotify.
/// Concatenates the op name, vars and other params into a single string.
pub(super) fn build_get_url<T>(
    operation_name: &str,
    variables: &T,
    persisted_query_hash: &str,
) -> Result<String>
where
    T: ?Sized + serde::Serialize + Debug,
{
    let variables = match serde_json::to_string(variables) {
        Ok(v) => v,
        Err(e) => {
            log!("Failed to serialize variables for {:?}", variables);
            log!("{e}");
            return Err(RetryAfter::Never);
        }
    };

    let variables = urlencoding::encode(&variables).to_string();

    let url = &[
        "https://api-partner.spotify.com/pathfinder/v1/query?operationName=",
        operation_name,
        "&variables=",
        &variables,
    ]
    .concat();

    let persisted_query = [
        r#"{"persistedQuery":{"version":1,"sha256Hash":""#,
        persisted_query_hash,
        r#""}}""#,
    ]
    .concat();

    let url = &[url, "&extensions=", &urlencoding::encode(&persisted_query)].concat();

    // log!("URL:");
    // log!("{url}");

    Ok(url.to_owned())
}
