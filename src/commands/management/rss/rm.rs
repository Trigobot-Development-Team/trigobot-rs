use crate::State;

use crate::commands::management::rss::rm_feed_message;
use crate::env::*;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
async fn rm(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() != 1 {
        msg.reply(ctx, "Need a feed name").await?;
    } else {
        let name = args.single::<String>().unwrap();

        {
            let mut lock = ctx.data.write().await;

            let mut state = lock
                .get_mut::<State>()
                .expect("No state provided")
                .write()
                .await;

            if state.get_feeds().contains_key(&name) {
                let feed = state.get_mut_feeds().remove(&name).unwrap();

                rm_feed_message(ctx, feed.get_message()).await?;

                // Remove from reaction messages
                state.get_mut_messages().remove(&feed.get_message().0);

                match State::save_to_file(&get_var(Variables::StateFile), &state) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Error saving state: {}", e);

                        msg.reply(ctx, "Erro ao executar comando").await?;

                        return Ok(());
                    }
                }

                msg.reply(ctx, format!("Feed {} removido com sucesso", name))
                    .await?;
            } else {
                msg.reply(ctx, "Feed inexistente").await?;
            }
        }
    }

    Ok(())
}
