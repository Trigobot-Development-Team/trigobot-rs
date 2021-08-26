use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

const LINK: &str = "https://github.com/afonsocrg/MEIC-feedback";

#[command]
#[description("Shows you were are the trap courses")]
async fn traps(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(
        ctx,
        MessageBuilder::new()
            .push_line(format!(
                "Podes encontrar todas as traps (e não traps) em {}",
                LINK
            ))
            .push_line("Podes ainda contribuir com o teu próprio feedback")
            .push_line("")
            .push_spoiler("WARNING: MEIC É TRAP")
            .build(),
    )
    .await?;

    Ok(())
}
