mod ai_models;
mod commands;
mod database;
mod error;
mod handlers;

use crate::commands::Commands;
use crate::database::db_utils::get_pool;
use crate::error::Error;
use crate::handlers::callback::callback_handler;
use crate::handlers::inline::inline_query_handler;
use askama_axum::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use block_mesh_common::env::load_dotenv::load_dotenv;
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use database_utils::utils::migrate::migrate;
use http::StatusCode;
use logger_general::tracing::setup_tracing_stdout_only_with_sentry;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::{env, mem, process};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::dptree::case;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::Bot;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
}

fn bot_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    // https://docs.rs/teloxide/latest/teloxide/dispatching/index.html

    let command_handler = teloxide::filter_command::<Commands, _>().branch(
        case![State::Start]
            .branch(case![Commands::Help].endpoint(handlers::help::help))
            // .branch(case![Commands::SelectMode].endpoint(handlers::select_mode::select_mode))
            .branch(case![Commands::SelectModel].endpoint(handlers::select_model::select_model))
            .branch(case![Commands::Info].endpoint(handlers::info::info))
            .branch(case![Commands::Start].endpoint(handlers::start::start)),
    );

    let message_handler = Update::filter_message().branch(command_handler);

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler))
        .branch(
            Update::filter_chosen_inline_result()
                .endpoint(handlers::chosen_inline_result::chosen_inline_result_handler),
        )
        .branch(Update::filter_message().endpoint(handlers::message::message_handler))
}

#[tracing::instrument(name = "health", skip_all)]
pub async fn health(Extension(pool): Extension<PgPool>) -> Result<impl IntoResponse, Error> {
    let mut transaction = create_txn(&pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "version", skip_all)]
pub async fn version() -> impl IntoResponse {
    (StatusCode::OK, env!("CARGO_PKG_VERSION"))
}

fn main() {
    let sentry_layer = env::var("SENTRY_LAYER")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
    let sentry_url = env::var("SENTRY_TG").unwrap_or_default();
    let sentry_sample_rate = env::var("SENTRY_SAMPLE_RATE")
        .unwrap_or("0.1".to_string())
        .parse()
        .unwrap_or(0.1);
    if sentry_layer {
        let _guard = sentry::init((
            sentry_url,
            sentry::ClientOptions {
                debug: env::var("APP_ENVIRONMENT").unwrap_or_default() == "local",
                sample_rate: sentry_sample_rate,
                traces_sample_rate: sentry_sample_rate,
                release: sentry::release_name!(),
                ..Default::default()
            },
        ));
        mem::forget(_guard);
    }

    let _ = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run().await });
    tracing::error!("tg privacy bot, exiting with exit code 1");
    process::exit(1);
}

#[tracing::instrument(name = "run_server", skip_all)]
pub async fn run_server(listener: TcpListener, app: Router<()>) -> std::io::Result<()> {
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}

#[tracing::instrument(name = "dispatch_bot", skip_all)]
pub async fn dispatch_bot(bot: Bot) {
    Dispatcher::builder(bot.clone(), bot_schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .default_handler(|upd| async move {
            eprintln!("\nUnhandled update: {:?}\n", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "\nAn error has occurred in the dispatcher\n",
        ))
        .build()
        .dispatch()
        .await
}

#[tracing::instrument(name = "run", skip_all, ret, err)]
async fn run() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only_with_sentry();
    let bot = Bot::from_env();
    bot.set_my_commands(Commands::bot_commands()).await?;
    let db_pool = get_pool().await;
    let env = env::var("APP_ENVIRONMENT")?;
    migrate(db_pool, env).await?;
    println!("Dispatching bot");

    let router = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/version", get(version));
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .nest("/", router)
        .layer(cors)
        .layer(Extension(db_pool.clone()));

    let port = env::var("PORT").unwrap_or("8001".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    let server_task = run_server(listener, app);
    let bot_task = dispatch_bot(bot);

    tokio::select! {
        o = bot_task => panic!("bot_task exit {:?}", o),
        o = server_task => panic!("server_task exit {:?}", o)
    }
}
