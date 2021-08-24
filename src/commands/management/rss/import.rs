use crate::commands::management::rss::{
    add_feed_channel, add_feed_message, add_feed_role, rm_feed_message,
};
use crate::env::*;
use crate::{Feed, State};

use std::collections::{HashMap, HashSet};

use serde::Deserialize;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;

#[derive(Deserialize)]
struct SimpleFeed {
    link: String,
    updated: u64,
}

#[command]
#[only_in(guilds)]
async fn import(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(ctx, "Need the json to import").await?;
    } else {
        let new_feeds: HashMap<String, SimpleFeed> = match serde_json::from_str(args.rest()) {
            Ok(v) => v,
            Err(_) => {
                msg.reply(ctx, "JSON inválido").await?;

                return Ok(());
            }
        };

        let mut messages_to_remove: HashSet<u64> = HashSet::new();
        let mut messages_to_add: HashMap<u64, u64> = HashMap::new();

        let category = {
            let lock = ctx.data.read().await;

            let state = lock.get::<State>().expect("No state provided").read().await;

            state.category
        };

        if let Some(category) = category {
            let mut lock = ctx.data.write().await;

            let mut state = lock
                .get_mut::<State>()
                .expect("No state provided")
                .write()
                .await;

            {
                let feeds = state.get_mut_feeds();
                // Update feeds
                for (name, f) in new_feeds {
                    if let Some(old) = feeds.get_mut(&name) {
                        let message = old.get_message();
                        messages_to_remove.insert(message.0);

                        rm_feed_message(ctx, message).await?;

                        let role = old.get_role();

                        let message =
                            add_feed_message(ctx, &name, role, ChannelId(category)).await?;

                        old.set_link(f.link);
                        old.set_update(f.updated);
                        old.set_message(message.0);

                        messages_to_add.insert(message.0, role.0);
                    } else {
                        let guild = msg.guild_id.unwrap();

                        // Create channel and role (if they don't exist)
                        let role = add_feed_role(ctx, &guild, &name).await?;
                        let channel =
                            add_feed_channel(ctx, &guild, &name, role.id, ChannelId(category))
                                .await?;

                        // Create reaction-role message
                        let message = add_feed_message(ctx, &name, role.id, channel.id).await?;

                        feeds.insert(
                            name.to_owned(),
                            Feed::new(
                                name.to_owned(),
                                f.link,
                                role.id.0,
                                channel.id.0,
                                message.0,
                                Some(f.updated),
                            ),
                        );

                        messages_to_add.insert(message.0, role.id.0);
                    }
                }
            }

            {
                let messages = state.get_mut_messages();

                // Update messages
                if !messages_to_remove.is_empty() {
                    messages_to_remove.iter().for_each(|m| {
                        messages.remove(m);
                    });
                }

                messages.extend(messages_to_add);
            }

            match State::save_to_file(&get_var(Variables::StateFile), &state) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error saving state: {}", e);

                    msg.reply(ctx, "Erro ao executar comando").await?;

                    return Ok(());
                }
            }

            msg.reply(ctx, "Feeds importados com sucesso").await?;
        } else {
            msg.reply(ctx, "Nenhuma categoria selecionada").await?;
        }
    }

    Ok(())
}
