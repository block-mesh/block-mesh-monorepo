use std::fs;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber(
    name: &str,
    env_filter: &str,
    sink: impl for<'a> MakeWriter<'a> + 'static + Send + Sync,
    with_file: bool,
) -> Box<dyn Subscriber + Send + Sync> {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name.into(), sink);
    let r = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    if with_file {
        fs::create_dir_all("./logs").expect("failed to create logs dir");
        fs::remove_dir_all("./logs").expect("failed to delete logs dir content");
        let appender = RollingFileAppender::builder()
            .rotation(Rotation::MINUTELY)
            .filename_prefix("app")
            .filename_suffix("log")
            .build("./logs")
            .expect("initializing rolling file appender failed");
        let file_layer = Layer::new()
            .with_writer(appender)
            .with_file(true)
            .with_ansi(false)
            .with_target(true);
        Box::new(r.with(file_layer))
    } else {
        Box::new(r)
    }
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
