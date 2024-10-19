use crate::HandlerResult;
use teloxide::prelude::*;
use teloxide::Bot;

pub async fn message_handler(bot: Bot, msg: Message) -> HandlerResult {
    println!("\nmessage_handler: {:?}\n", msg);
    let _message = msg.text().unwrap_or_default().to_string();
    let _new_msg = bot
        .send_message(
            msg.chat.id,
            "Please setup your Telegram username first and retry",
        )
        .await?;
    Ok(())
}
