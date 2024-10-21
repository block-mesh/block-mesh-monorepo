use crate::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

const HELP_TEXT: &str = r#"
/reset_on_each_message - [Default] Reset chat context for each message
/reset_on_model_change - Reset chat context on model change
/reset - Reset current chat context
/keep -  Will keep the context until it reaches the limit
/select_model - Select a different model
/info - Show current settings
/help - This message
"#;

pub async fn help(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let _ = bot.send_message(msg.chat.id, HELP_TEXT).await?;
    Ok(())
}
