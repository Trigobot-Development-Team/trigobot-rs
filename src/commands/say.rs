use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

#[command]
#[description("Make the bot say something")]
#[usage("<message>")]
#[example("Hi")]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let channel = msg.channel_id;

    // Can't delete messages in DMs
    if !msg.is_private() {
        match msg.delete(&ctx).await {
            Ok(_) => (),
            Err(_) => eprintln!(
                "[SAY] Couldn't delete message from {} on channel {}",
                msg.author.name, msg.channel_id
            ),
        };
    }

    channel
        .say(ctx, MessageBuilder::new().push(args.message()).build())
        .await?;

    Ok(())
}
