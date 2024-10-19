use crate::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

pub async fn help(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let _ = bot.send_message(msg.chat.id, "HELP!".to_string()).await?;
    Ok(())
}
