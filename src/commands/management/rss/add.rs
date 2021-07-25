use crate::Feed;
use crate::State;

use crate::commands::management::{add_feed_channel, add_feed_role};
use crate::env::*;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::{Message, ReactionType};
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

                let state = lock.get::<State>().expect("No state provided");

                state.category
            };

            if let Some(category) = category {
                let guild = msg.guild_id.unwrap();

                // Create channel and role (if they don't exist)
                let role = add_feed_role(ctx, &guild, &name).await?;
                let channel = add_feed_channel(ctx, &guild, &name, role.id, category).await?;

                // Create reaction-role message
                let reaction = get_var(Variables::ReactionRole);

                let message = ChannelId(
                    get_var(Variables::ReactionRolesChannel)
                        .parse::<u64>()
                        .expect("React roles' channel id is not valid!"),
                )
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.title(format!("[{}] Cadeira disponível / Course available", name));

                        // Can't use const's as format strings
                        e.description(format!("**[PT]**\n\nSe vai fazer a cadeira **{}** reage com {} para teres acesso ao role {}, ao canal {} e receberes notificações de anúncios\nPara remover tudo isto só precisas de remover a reação\n\n**[EN]**\n\nIf you are enrolling in **{}** react with {} to get access to the role {}, the channel {} and to receive announcements' notifications\nTo quit all of this, just remove the reaction", name, reaction, role, channel, name, reaction, role, channel));

                        e
                    });
                    m.reactions(vec![ReactionType::Unicode(reaction)]);

                    m
                })
                .await?;

                {
                    let mut lock = ctx.data.write().await;

                    let state = lock.get_mut::<State>().expect("No state provided");

                    state.get_mut_feeds().insert(
                        name.to_owned(),
                        Feed::new(
                            name.to_owned(),
                            link.to_owned(),
                            role.id,
                            channel.id,
                            message.id,
                            None,
                        ),
                    );

                    state.get_mut_messages().insert(message.id, role.id);
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
