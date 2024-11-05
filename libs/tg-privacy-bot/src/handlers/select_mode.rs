use crate::database::calls::get_or_create_usage::get_or_create_usage;
use crate::database::calls::get_or_create_user::get_or_create_user;
use crate::database::calls::get_or_create_user_settings::get_or_create_user_settings;
use crate::database::models::message_mode::MessageMode;
use crate::{HandlerResult, MyDialogue};
use database_utils::utils::connection::get_pg_pool;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::Bot;

#[allow(dead_code)]
#[tracing::instrument(name = "select_mode", skip(bot, _dialogue))]
pub async fn select_mode(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let pool = get_pg_pool().await;
    let mut transaction = create_txn(&pool).await?;

    match msg.from {
        Some(ref from) => {
            let username = from.username.clone().unwrap_or_default();
            let tg_id = from.id.0;
            let _message = msg.text().unwrap_or_default().to_string();
            let user = get_or_create_user(&mut transaction, tg_id as i64, &username).await?;
            let _ = get_or_create_usage(&mut transaction, &user.id).await?;
            let _ = get_or_create_user_settings(&mut transaction, &user.id).await?;
            commit_txn(transaction).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Cannot get user data")
                .await?;
        }
    }

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            "Reset on each message",
            format!("select_mode_{}", MessageMode::ResetOnEachMessage),
        )],
        vec![InlineKeyboardButton::callback(
            "Reset on model change",
            format!("select_mode_{}", MessageMode::ResetOnModelChange),
        )],
        vec![InlineKeyboardButton::callback(
            "Keep always",
            format!("select_mode_{}", MessageMode::KeepAlways),
        )],
    ]);

    // Send a message with the inline keyboard
    bot.send_message(msg.chat.id, "Please operation mode:")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}
