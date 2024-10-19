mod commands;
mod handlers;
mod models;

use crate::commands::Commands;
use crate::handlers::callback::callback_handler;
use crate::handlers::inline::inline_query_handler;
use block_mesh_common::env::load_dotenv::load_dotenv;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::dptree::case;
use teloxide::prelude::*;
use teloxide::types::BotCommand;
use teloxide::Bot;

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

#[tokio::main]
async fn main() {
    load_dotenv();
    let bot = Bot::from_env();
    println!("Dispatching bot");
    Dispatcher::builder(bot.clone(), bot_schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .default_handler(|upd| async move {
            println!("\nUnhandled update: {:?}\n", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "\nAn error has occurred in the dispatcher\n",
        ))
        .build()
        .dispatch()
        .await;
}
