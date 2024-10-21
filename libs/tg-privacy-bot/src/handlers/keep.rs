use crate::{HandlerResult, MyDialogue};
use teloxide::prelude::*;
use teloxide::Bot;

pub async fn keep(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let _ = bot.send_message(msg.chat.id, "keep").await?;
    Ok(())
}
