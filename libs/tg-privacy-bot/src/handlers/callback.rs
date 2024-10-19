use crate::HandlerResult;
use teloxide::prelude::CallbackQuery;
use teloxide::prelude::*;
use teloxide::Bot;

/// When it receives a callback from a button it edits the message with all
/// those buttons writing a text with the selected Debian version.
///
/// **IMPORTANT**: do not send privacy-sensitive data this way!!!
/// Anyone can read data stored in the callback button.
pub async fn callback_handler(bot: Bot, q: CallbackQuery) -> HandlerResult {
    println!("\nReceived callback query: {:?}\n", q);
    Ok(())
}
