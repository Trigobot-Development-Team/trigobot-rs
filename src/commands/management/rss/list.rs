use crate::State;

use chrono::{DateTime, NaiveDateTime, Utc};

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let list = {
        let lock = ctx.data.read().await;

        let state = lock.get::<State>().expect("No state provided").read().await;

        let mut msg = MessageBuilder::new();

        for f in state.get_feeds().values().into_iter() {
            msg.push_bold(f.get_name())
                .push(": ")
                .push_line(f.get_link())
                .push("Última atualização: ")
                .push_line(
                    DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(f.get_update() as i64, 0),
                        Utc,
                    )
                    .format("%Y/%m/%d %T %Z"),
                );
        }

        msg.build()
    };

    msg.reply(ctx, list).await?;

    Ok(())
}
