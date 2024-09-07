use crate::State;

use chrono::DateTime;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

const MESSAGE_MAX_LENGTH: usize = 2000;

#[command]
#[description("Get feed status")]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let mut messages: Vec<String> = Vec::new();

    {
        let lock = ctx.data.read().await;

        let state = lock.get::<State>().expect("No state provided").read().await;

        let mut cur = String::new();

        for f in state.get_feeds().values() {
            let tmp = MessageBuilder::new()
                .push_bold(f.get_name())
                .push(": ")
                .push_line(f.get_link())
                .push("Última atualização: ")
                .push_line(
                    DateTime::from_timestamp(f.get_update() as i64, 0)
                        .ok_or("invalid last update")?
                        .format("%Y/%m/%d %T %Z"),
                )
                .build();

            if cur.len() + tmp.len() > MESSAGE_MAX_LENGTH {
                messages.push(cur);
                cur = tmp;
            } else {
                cur.push_str(&tmp);
            }
        }

        if !cur.is_empty() {
            messages.push(cur);
        }
    };

    for m in messages {
        msg.channel_id.say(ctx, m).await?;
    }

    Ok(())
}
