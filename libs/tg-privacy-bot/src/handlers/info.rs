use crate::database::calls::get_or_create_usage::get_or_create_usage;
use crate::database::calls::get_or_create_user::get_or_create_user;
use crate::database::calls::get_or_create_user_settings::get_or_create_user_settings;
use crate::database::db_utils::get_pool;
use crate::{HandlerResult, MyDialogue};
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use teloxide::prelude::*;
use teloxide::Bot;

#[tracing::instrument(name = "info", skip(bot, _dialogue))]
pub async fn info(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let pool = get_pool().await;
    let mut transaction = create_txn(pool).await?;
    match msg.from {
        Some(ref from) => {
            let username = from.username.clone().unwrap_or_default();
            let tg_id = from.id.0;
            let _message = msg.text().unwrap_or_default().to_string();
            let user = get_or_create_user(&mut transaction, tg_id as i64, &username).await?;
            let usage = get_or_create_usage(&mut transaction, &user.id).await?;
            let user_settings = get_or_create_user_settings(&mut transaction, &user.id).await?;
            commit_txn(transaction).await?;
            // let response = format!(
            //     r#"
            //     Mode: {} | Model Name: {}
            //     "#,
            //     user_settings.message_mode, user_settings.model_name
            // );
            let response = format!(
                r#"
                Model Name: {} | Usage {} / {}
                "#,
                user_settings.model_name, usage.usage, usage.usage_limit
            );
            let _r = bot.send_message(msg.chat.id, response).await;
        }
        None => {
            bot.send_message(msg.chat.id, "Cannot get user data")
                .await?;
        }
    }
    Ok(())
}
