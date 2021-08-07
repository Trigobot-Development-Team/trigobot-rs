use crate::model::update_all_feeds;
use crate::State;

use std::sync::Arc;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

#[command]
async fn rss(ctx: &Context, msg: &Message) -> CommandResult {
    {
        let mut lock = ctx.data.write().await;

        let mut state = lock
            .get_mut::<State>()
            .expect("No state provided")
            .write()
            .await;

        update_all_feeds((&Arc::clone(&ctx.cache), &*ctx.http), &mut state).await?;
    }

    msg.reply(
        ctx,
        MessageBuilder::new()
            .push("Feeds atualizados com sucesso")
            .build(),
    )
    .await?;

    Ok(())
}
