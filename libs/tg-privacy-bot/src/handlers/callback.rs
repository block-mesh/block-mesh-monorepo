use crate::database::calls::get_or_create_usage::get_or_create_usage;
use crate::database::calls::get_or_create_user::get_or_create_user;
use crate::database::calls::get_or_create_user_settings::get_or_create_user_settings;
use crate::database::calls::update_user_settings_message_mode::update_user_settings_message_mode;
use crate::database::calls::update_user_settings_model_name::update_user_settings_model_name;
use crate::database::models::message_mode::MessageMode;
use crate::HandlerResult;
use ai_interfaces::models::base::ModelName;
use database_utils::utils::connection::get_pg_pool;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use teloxide::prelude::*;
use teloxide::Bot;

/// When it receives a callback from a button it edits the message with all
/// those buttons writing a text with the selected Debian version.
///
/// **IMPORTANT**: do not send privacy-sensitive data this way!!!
/// Anyone can read data stored in the callback button.
///
#[tracing::instrument(name = "callback_handler", skip(bot, query))]
pub async fn callback_handler(bot: Bot, query: CallbackQuery) -> HandlerResult {
    let pool = get_pg_pool().await;
    let mut transaction = create_txn(&pool).await?;

    let from = query.from;
    let username = from.username.clone().unwrap_or_default();
    let tg_id = from.id.0;
    let user = get_or_create_user(&mut transaction, tg_id as i64, &username).await?;
    let _ = get_or_create_usage(&mut transaction, &user.id).await?;
    let user_settings = get_or_create_user_settings(&mut transaction, &user.id).await?;

    if let Some(data) = query.data {
        let data = data.as_str().to_string();
        if data.starts_with("select_mode_") {
            let message_mode =
                MessageMode::from(data.replace("select_mode_", "").trim().to_string());
            update_user_settings_message_mode(&mut transaction, &user_settings.id, &message_mode)
                .await?;
            if let Some(message) = query.message {
                bot.send_message(
                    message.chat().id,
                    format!("Changed to mode: {}", message_mode),
                )
                .await?;
            }
            // Acknowledge the callback query
            bot.answer_callback_query(query.id).await?;
        } else if data.starts_with("select_model_") {
            let model_name = ModelName::from(data.replace("select_model_", "").trim().to_string());
            update_user_settings_model_name(&mut transaction, &user_settings.id, &model_name)
                .await?;
            if let Some(message) = query.message {
                bot.send_message(
                    message.chat().id,
                    format!("Changed to model: {}", model_name),
                )
                .await?;
            }
            // Acknowledge the callback query
            bot.answer_callback_query(query.id).await?;
        }
    }
    commit_txn(transaction).await?;
    Ok(())
}
