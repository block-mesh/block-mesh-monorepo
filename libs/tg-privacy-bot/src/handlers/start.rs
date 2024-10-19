use crate::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

pub async fn start(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    println!("\nstart msg : {:?}\n", msg);

    Ok(())
}
