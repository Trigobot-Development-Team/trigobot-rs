use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

#[command]
async fn anunciar(ctx: &Context, msg: &Message) -> CommandResult {
    let user = msg.author.clone();
    let channel = msg.channel_id;

    println!(
        "[{}] on {}: {}",
        msg.author.name,
        if msg.is_private() {
            "DM".to_string()
        } else {
            msg.channel_id.to_string()
        },
        msg.content
    );

    // Can't delete messages in DMs
    if !msg.is_private() {
        match msg.delete(&ctx).await {
            Ok(_) => (),
            Err(_) => eprintln!(
                "[ANUNCIAR] Couldn't delete message from {} on channel {}",
                msg.author.name, msg.channel_id
            ),
        };
    }

    channel
        .say(
            ctx,
            MessageBuilder::new()
                .push("Vai pó caralho")
                .mention(&user)
                .build(),
        )
        .await?;

    Ok(())
}
