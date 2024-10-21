use crate::{HandlerResult, MyDialogue};
use teloxide::prelude::*;
use teloxide::Bot;

pub async fn reset(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let _ = bot.send_message(msg.chat.id, "reset").await?;
    Ok(())
}
