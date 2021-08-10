use crate::{Feed, State};

use crate::commands::management::rss::{
    add_feed_channel, add_feed_message, add_feed_role, rm_feed_message,
};
use crate::env::*;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;

#[command]
#[only_in(guilds)]
async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() != 2 {
        msg.reply(ctx, "Need a feed name and link").await?;
    } else {
        let name = args.single::<String>().unwrap();
        let link = args.single::<String>().unwrap();

        if Feed::test_link(&link).await {
            let category = {
                let lock = ctx.data.read().await;

                let state = lock.get::<State>().expect("No state provided").read().await;

                state.category
            };

            if let Some(category) = category {
                let guild = msg.guild_id.unwrap();

                // Create channel and role (if they don't exist)
                let role = add_feed_role(ctx, &guild, &name).await?;
                let channel =
                    add_feed_channel(ctx, &guild, &name, role.id, ChannelId(category)).await?;

                // Create reaction-role message
                let message = add_feed_message(ctx, &name, role.id, channel.id).await?;

                {
                    let mut lock = ctx.data.write().await;

                    let mut state = lock
                        .get_mut::<State>()
                        .expect("No state provided")
                        .write()
                        .await;

                    state.get_mut_messages().insert(message.0, role.id.0);
                    if let Some(old) = state.get_mut_feeds().insert(
                        name.to_owned(),
                        Feed::new(
                            name.to_owned(),
                            link.to_owned(),
                            role.id.0,
                            channel.id.0,
                            message.0,
                            None,
                        ),
                    ) {
                        rm_feed_message(ctx, old.get_message()).await?;

                        state.get_mut_messages().remove(&old.get_message().0);
                    }

                    match State::save_to_file(&get_var(Variables::StateFile), &state) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("Error saving state: {}", e);

                            msg.reply(ctx, "Erro ao executar comando").await?;

                            return Ok(());
                        }
                    }
                }

                msg.reply(ctx, "Feed adicionado com sucesso").await?;
            } else {
                msg.reply(ctx, "Nenhuma categoria selecionada").await?;
            }
        } else {
            msg.reply(ctx, "Link de feed inválido").await?;
        }
    }

    Ok(())
}
