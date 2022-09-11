use crate::model::update_all_feeds;
use crate::State;

use std::sync::Arc;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult, CommandError};
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

#[command]
#[description("Update all feeds")]
async fn rss(ctx: &Context, msg: &Message) -> CommandResult {
    let update_res = {
        let mut lock = ctx.data.write().await;

        let mut state = lock
            .get_mut::<State>()
            .expect("No state provided")
            .write()
            .await;

        update_all_feeds((&Arc::clone(&ctx.cache), &*ctx.http), &mut state).await
    };

    match update_res {
        Ok(_) => {
            msg.reply(
                ctx,
                MessageBuilder::new()
                    .push("Feeds atualizados com sucesso")
                    .build(),
            )
            .await?;
        }
        Err(errors) => {
            for (i, (feed_name, err)) in errors.iter().enumerate() {
                msg.reply(
                    ctx,
                    MessageBuilder::new()
                        .push(format!("Erro {}/{} a atualizar feed {}:\n", i+1, errors.len(), feed_name))
                        .push(err.to_string())
                        .build(),
                )
                .await?;
            }
        }
    }


    Ok(())
}
