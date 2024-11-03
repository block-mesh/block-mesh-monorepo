use crate::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

pub async fn start(_bot: Bot, _dialogue: MyDialogue, _msg: Message) -> HandlerResult {
    Ok(())
}
