use crate::State;

use std::collections::HashSet;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::id::RoleId;

#[command]
#[description("Remove roles from the user")]
#[usage("<roles>")]
#[example("IAC IEI")]
#[min_args(1)]
#[only_in(guilds)]
async fn rm(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let roles: HashSet<String> = args.rest().split(' ').map(|s| s.to_string()).collect();

    let feeds = {
        let lock = ctx.data.read().await;

        let state = lock.get::<State>().expect("No state provided").read().await;

        state.get_feeds_simple()
    };

    let roles_to_remove: Vec<RoleId> = roles
        .intersection(&feeds.keys().cloned().collect())
        .map(|n| feeds.get(n).unwrap().to_owned())
        .collect();

    if !roles_to_remove.is_empty() {
        msg.member(ctx)
            .await?
            .remove_roles(ctx, &roles_to_remove[..])
            .await?;
        msg.reply(ctx, "Role(s) removed").await?;
    } else {
        msg.reply(ctx, "No valid roles detected").await?;
    }

    Ok(())
}
