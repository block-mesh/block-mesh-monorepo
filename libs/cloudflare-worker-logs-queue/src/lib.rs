use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use wasm_bindgen::JsValue;
use worker::*;

const PRODUCER: &str = "rawlog";
const CONSUMER: &str = "rawlog";

#[derive(Serialize, Deserialize, Debug)]
pub struct Wrapper(Map<String, Value>);

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

    let mut array: Vec<Map<String, Value>> = Vec::new();
    if body.is_object() {
        array.push(body.as_object_mut().unwrap().to_owned());
    } else if body.is_array() {
        array.extend(
            body.as_array_mut()
                .unwrap()
                .iter_mut()
                .filter(|i| i.is_object())
                .map(|i| i.as_object_mut().unwrap().to_owned())
                .collect::<Vec<Map<String, Value>>>(),
        );
    }
    let timestamp = chrono::offset::Utc::now().timestamp();

    array.iter_mut().for_each(|obj| {
        obj.insert("cloudflare-timestamp".to_string(), Value::from(timestamp));
    });
    let messages: Vec<SendMessage<JsValue>> = array
        .iter()
        .map(|i| {
            let string = serde_json::to_string(&i).unwrap();
            let js_string = JsValue::from_str(&string);
            RawMessageBuilder::new(js_string).build_with_content_type(QueueContentType::Json)
        })
        .collect();
    let msg_builder = BatchMessageBuilder::new()
        .messages(messages)
        .delay_seconds(1)
        .build();
    raw_messages_queue.send_raw_batch(msg_builder).await?;
    Response::empty()
}

// Consumes messages from queue
#[event(queue)]
pub async fn main(message_batch: MessageBatch<Value>, env: Env, _: Context) -> Result<()> {
    let url = env.secret("log_url").unwrap().to_string();
    match message_batch.queue().as_str() {
        CONSUMER => {
            let messages: Vec<Wrapper> = message_batch
                .iter()
                .map(|message| {
                    let raw_str = message.unwrap().raw_body().as_string().unwrap();
                    let wrapper: Wrapper = serde_json::from_str(&raw_str).unwrap();
                    wrapper
                })
                .collect();
            match reqwest::Client::new()
                .post(&url)
                .header("Content-Type", "application/json")
                .json(&messages)
                .send()
                .await
            {
                Ok(_) => {
                    console_log!("Successfully sent messages {} to {}", messages.len(), url);
                    let j = serde_json::to_string(&messages).unwrap();
                    console_log!("Messages => {:#?}", j);
                    message_batch.ack_all()
                }
                Err(e) => console_error!("Error {}", e),
            }
        }
        _ => {
            console_error!("Unknown queue: {}", message_batch.queue());
        }
    }
    Ok(())
}
