use crate::database::calls::get_or_create_usage::get_or_create_usage;
use crate::database::calls::get_or_create_user::get_or_create_user;
use crate::database::calls::increment_usage::increment_usage;
use crate::models::open_ai::ask;
use crate::HandlerResult;
use database_utils::utils::connection::get_pg_pool;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use teloxide::prelude::*;
use teloxide::Bot;

pub async fn message_handler(bot: Bot, msg: Message) -> HandlerResult {
    println!("\nmessage_handler: {:?}\n", msg);
    let pool = get_pg_pool().await;
    let mut transaction = create_txn(&pool).await?;
    match msg.from {
        Some(ref from) => {
            let username = from.username.clone().unwrap_or_default();
            let tg_id = from.id.0;
            let message = msg.text().unwrap_or_default().to_string();
            println!("message received: {:?}\n", message);
            let user = get_or_create_user(&mut transaction, tg_id as i64, &username).await?;
            let usage = get_or_create_usage(&mut transaction, &user.id).await?;
            if usage.over_limit() {
                bot.send_message(msg.chat.id, "Usage limit exceeded")
                    .await?;
                return Ok(());
            }
            increment_usage(&mut transaction, &usage.id).await?;
            commit_txn(transaction).await?;
            let response = ask(message).await?;
            bot.send_message(msg.chat.id, response).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Cannot get user data")
                .await?;
        }
    }
    Ok(())
}
