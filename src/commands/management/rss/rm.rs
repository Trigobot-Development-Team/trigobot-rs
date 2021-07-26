use crate::State;

use crate::env::*;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;

#[command]
async fn rm(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() != 1 {
        msg.reply(ctx, "Need a feed name").await?;
    } else {
        let name = args.single::<String>().unwrap();

        {
            let mut lock = ctx.data.write().await;

            let state = lock.get_mut::<State>().expect("No state provided");

            if state.get_feeds().contains_key(&name) {
                let feed = state.get_mut_feeds().remove(&name).unwrap();

                ChannelId(
                    get_var(Variables::ReactionRolesChannel)
                        .parse::<u64>()
                        .expect("React roles' channel id is not valid"),
                )
                .delete_message(ctx, feed.get_message())
                .await?;

                match State::save_to_file(&get_var(Variables::StateFile), state) {
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
