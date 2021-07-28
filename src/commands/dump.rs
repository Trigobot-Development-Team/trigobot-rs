use crate::State;

use serde_json::json;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
async fn dump(ctx: &Context, msg: &Message) -> CommandResult {
    let dump = {
        let lock = ctx.data.read().await;

        let state = lock.get::<State>().expect("No state provided");

        json!(state)
    };

    msg.reply(&ctx, dump).await?;

    Ok(())
}
