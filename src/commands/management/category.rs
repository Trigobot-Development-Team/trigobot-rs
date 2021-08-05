use crate::env::*;
use crate::State;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;

#[command]
async fn category(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.len() != 1 {
        msg.reply(ctx, "Need a valid Category ID as argument")
            .await?;
    } else {
        let category = ChannelId(match args.parse::<u64>() {
            Ok(val) => val,
            Err(_) => {
                msg.reply(ctx, "Couldn't parse category id").await?;

                return Ok(());
            }
        });

        {
            let mut lock = ctx.data.write().await;

            let mut state = lock
                .get_mut::<State>()
                .expect("No state provided")
                .lock()
                .await;

            state.set_category(category.0);

            match State::save_to_file(&get_var(Variables::StateFile), &state) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error saving state: {}", e);

                    msg.reply(ctx, "Erro ao executar comando").await?;

                    return Ok(());
                }
            };
        }

        msg.reply(ctx, "Category changed successfully").await?;
    }

    Ok(())
}
