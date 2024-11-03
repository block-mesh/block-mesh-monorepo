use crate::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

// const HELP_TEXT: &str = r#"
// /select_mode - Select message context mode
// /select_model - Select a different model
// /info - Show current settings
// /help - This message
// "#;

const HELP_TEXT: &str = r#"
/select_model - Select a different model
/info - Show current settings
/help - This message
"#;

pub async fn help(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let _ = bot.send_message(msg.chat.id, HELP_TEXT).await?;
    Ok(())
}
