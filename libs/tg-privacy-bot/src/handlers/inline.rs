use teloxide::prelude::*;
use teloxide::Bot;

use crate::HandlerResult;

#[tracing::instrument(name = "inline_query_handler", skip(_bot, _q))]
pub async fn inline_query_handler(_bot: Bot, _q: InlineQuery) -> HandlerResult {
    // let choose_debian_version = InlineQueryResultArticle::new(
    //     "0",
    //     "Chose debian version",
    //     InputMessageContent::Text(InputMessageContentText::new("Debian versions:")),
    // )
    // .reply_markup(make_actions_keyboard());
    //
    // let choose_debian_version = "11";
    // bot.answer_inline_query(q.id, vec![choose_debian_version.into()])
    //     .await?;
    Ok(())
}
