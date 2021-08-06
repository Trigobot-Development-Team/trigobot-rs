use crate::State;

use serde_json::json;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

const MESSAGE_MAX_LENGTH: usize = 2000;
const ADDED_CHARS: usize = 12;
const TOTAL_CHARS: usize = MESSAGE_MAX_LENGTH - ADDED_CHARS;

#[command]
async fn dump(ctx: &Context, msg: &Message) -> CommandResult {
    let dump = {
        let lock = ctx.data.read().await;

        let state = lock.get::<State>().expect("No state provided").read().await;

        json!(*state).to_string()
    };

    let mut iter = dump.chars().peekable();

    while iter.peek().is_some() {
        msg.channel_id
            .say(
                &ctx,
                MessageBuilder::new()
                    .push_codeblock(
                        iter.by_ref().take(TOTAL_CHARS).collect::<String>(),
                        Some("JSON"),
                    )
                    .build(),
            )
            .await?;
    }

    Ok(())
}
