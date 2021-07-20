use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

#[command]
async fn email(ctx: &Context, msg: &Message) -> CommandResult {
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

    channel
        .say(
            ctx,
            MessageBuilder::new()
                .push("Usem o suporte de problemas, idiotas!")
                .build(),
        )
        .await?;

    Ok(())
}
