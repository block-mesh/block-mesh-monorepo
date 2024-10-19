use crate::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

pub async fn start(_bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    println!("\nstart msg : {:?}\n", msg);

    Ok(())
}
