use crate::database::models::model_name::ModelName;
use crate::{HandlerResult, MyDialogue};
use enum_iterator::all;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::Bot;

pub async fn select_model(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let model_names = all::<ModelName>().collect::<Vec<_>>();

    let model_names_keyboard = model_names
        .iter()
        .map(|model_name| {
            vec![InlineKeyboardButton::callback(
                model_name.to_string(),
                format!("select_model_{}", model_name),
            )]
        })
        .collect::<Vec<_>>();

    let keyboard = InlineKeyboardMarkup::new(model_names_keyboard);

    // Send a message with the inline keyboard
    bot.send_message(msg.chat.id, "Select model:")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}
