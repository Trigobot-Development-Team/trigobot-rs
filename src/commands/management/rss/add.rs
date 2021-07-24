use crate::Feed;
use crate::State;

use crate::commands::management::{add_feed_channel, add_feed_role};

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
#[only_in(guilds)]
async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() != 2 {
        msg.reply(ctx, "Need a feed name and link").await?;
    } else {
        let name = args.single::<String>().unwrap();
        let link = args.single::<String>().unwrap();

        if Feed::test_link(&link).await {
            let guild = msg.guild_id.unwrap();

            let role = add_feed_role(ctx, &guild, &name).await?;

            let category = {
                let lock = ctx.data.read().await;

                let state = lock.get::<State>().expect("No state provided");

                state.category
            };

            if let Some(category) = category {
                let channel = add_feed_channel(ctx, &guild, &name, role.id, category).await?;

                {
                    let mut lock = ctx.data.write().await;

                    let state = lock.get_mut::<State>().expect("No state provided");

                    state.get_feeds().insert(
                        name.to_owned(),
                        Feed::new(name.to_owned(), link.to_owned(), role, channel, None),
                    );
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
