use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
async fn undo(ctx: &Context, msg: &Message) -> CommandResult {
    let channel = msg.channel_id;

    // Can't delete messages in DMs
    if !msg.is_private() {
        match msg.delete(&ctx).await {
            Ok(_) => (),
            Err(_) => eprintln!(
                "[EMAIL] Couldn't delete message from {} on channel {}",
                msg.author.name, msg.channel_id
            ),
        };
    }

    let bot_id = ctx.cache.current_user_id().await;

    if let Some(message) = channel
        .messages(ctx, |b| b)
        .await?
        .iter()
        .find(|m| m.author.id == bot_id)
    {
        channel.delete_message(ctx, message.id).await?;
    }

    Ok(())
}
