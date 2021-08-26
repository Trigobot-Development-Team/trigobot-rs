use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

#[command]
#[description("Announce yourself to the world in a classy way")]
async fn announce(ctx: &Context, msg: &Message) -> CommandResult {
    let user = msg.author.clone();
    let channel = msg.channel_id;

    // Can't delete messages in DMs
    if !msg.is_private() {
        match msg.delete(&ctx).await {
            Ok(_) => (),
            Err(_) => eprintln!(
                "[ANNOUNCE] Couldn't delete message from {} on channel {}",
                msg.author.name, msg.channel_id
            ),
        };
    }

    channel
        .say(
            ctx,
            MessageBuilder::new()
                .push("Vai pó caralho ")
                .mention(&user)
                .build(),
        )
        .await?;

    Ok(())
}
