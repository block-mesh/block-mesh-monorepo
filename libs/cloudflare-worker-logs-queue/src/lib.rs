use serde_json::Value;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use wasm_bindgen::JsValue;
use worker::*;

const PRODUCER: &str = "rawlog";
const CONSUMER: &str = "rawlog";

#[event(start)]
fn start() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false) // Only partially supported across JavaScript runtimes
        .with_timer(UtcTime::rfc_3339()) // std::time is not available in browsers
        .with_writer(MakeConsoleWriter); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
}

// Produce messages to queue
#[event(fetch)]
async fn main(mut req: Request, env: Env, _: worker::Context) -> Result<Response> {
    if req.method() != Method::Post {
        return Response::error("Only accept POST requests", 400);
    }

    let raw_messages_queue = env.queue(PRODUCER)?;

    let mut body: Value = match req.json().await {
        Ok(json) => json,
        Err(e) => return Response::error(e.to_string(), 400),
    };

    let obj = match body.as_object_mut() {
        Some(object) => object,
        None => return Response::error("Body isn't an object", 400),
    };

    let timestamp = chrono::offset::Utc::now().timestamp();
    obj.insert("cloudflare-timestamp".to_string(), Value::from(timestamp));
    // Send a message with using a serializable struct
    let string = serde_json::to_string(obj).unwrap();
    raw_messages_queue
        .send_raw(
            // RawMessageBuilder has to be used as we should set content type of these raw messages
            RawMessageBuilder::new(JsValue::from_str(&string))
                .delay_seconds(1)
                .build_with_content_type(QueueContentType::Json),
        )
        .await?;

    Response::empty()
}

// Consumes messages from queue
#[event(queue)]
pub async fn main(message_batch: MessageBatch<Value>, _: Env, _: Context) -> Result<()> {
    match message_batch.queue().as_str() {
        CONSUMER => {
            for message in message_batch.raw_iter() {
                let value: Value = serde_wasm_bindgen::from_value(message.body()).unwrap();
                console_log!(
                    "Got raw message {:?}, with id {} and timestamp: {} - {:#?}",
                    message.body(),
                    message.id(),
                    message.timestamp().to_string(),
                    value
                );
            }
            message_batch.ack_all();
        }
        _ => {
            console_error!("Unknown queue: {}", message_batch.queue());
        }
    }

    Ok(())
}
