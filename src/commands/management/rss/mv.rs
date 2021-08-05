use crate::commands::management::rss::{add_feed_message, mv_channel, mv_role, rm_feed_message};
use crate::env::*;
use crate::State;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
#[only_in(guilds)]
async fn mv(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() != 2 {
        msg.reply(ctx, "Need the old name and a new name").await?;
    } else {
        let old_name = args.single::<String>().unwrap();
        let new_name = args.single::<String>().unwrap();

        {
            let mut lock = ctx.data.write().await;

            let mut state = lock
                .get_mut::<State>()
                .expect("No state provided")
                .write()
                .await;

            if state.get_feeds().contains_key(&old_name) {
                let feeds = state.get_mut_feeds();

                let mut feed = feeds.remove(&old_name).unwrap();
                let old_msg = feed.get_message();
                let role = feed.get_role();

                // Change reaction role message
                rm_feed_message(ctx, old_msg).await?;
                let new_msg = add_feed_message(ctx, &new_name, role, feed.get_channel()).await?;
                feed.set_message(new_msg.0);

                // Change name
                feed.set_name(new_name.clone());

                // Change role
                mv_role(ctx, msg.guild_id.unwrap(), role, new_name.clone()).await?;

                // Change channel
                mv_channel(ctx, feed.get_channel(), new_name.clone()).await?;

                feeds.insert(new_name.clone(), feed);

                // Change message in hashtable
                // (Needs to be separate from the rest as we can't have 2 mutable references to the same struct)
                let messages = state.get_mut_messages();
                messages.remove(&old_msg.0);
                messages.insert(new_msg.0, role.0);

                match State::save_to_file(&get_var(Variables::StateFile), &state) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Error saving state: {}", e);

                        msg.reply(ctx, "Erro ao executar comando").await?;

                        return Ok(());
                    }
                }

                msg.reply(
                    ctx,
                    format!("Feed {} renomeado para {} com sucesso", old_name, new_name),
                )
                .await?;
            } else {
                msg.reply(ctx, "Feed inexistente").await?;
            }
        }
    }

    Ok(())
}
