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
    let channel = msg.channel_id;

    // Can't delete messages in DMs
    if !msg.is_private() {
        match msg.delete(&ctx).await {
            Ok(_) => (),
            Err(_) => eprintln!(
                "[RSS] Couldn't delete message from {} on channel {}",
                msg.author.name, msg.channel_id
            ),
        };
    }

    {
        let mut lock = ctx.data.write().await;

        let mut state = lock
            .get_mut::<State>()
            .expect("No state provided")
            .lock()
            .await;

        update_all_feeds((&Arc::clone(&ctx.cache), &*ctx.http), &mut state).await?;
    }

    channel
        .say(
            ctx,
            MessageBuilder::new()
                .push("Feeds atualizados com sucesso")
                .build(),
        )
        .await?;

    Ok(())
}
