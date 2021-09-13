use crate::State;

use std::collections::HashSet;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::id::RoleId;

#[command]
#[description("Remove all roles from the user")]
#[only_in(guilds)]
async fn clear(ctx: &Context, msg: &Message) -> CommandResult {
    let roles: HashSet<RoleId> = msg.member(&ctx).await?.roles.drain(..).collect();

    let feeds = {
        let lock = ctx.data.read().await;

        let state = lock.get::<State>().expect("No state provided").read().await;

        state.get_feeds_simple()
    };

    let roles_to_remove: Vec<RoleId> = roles
        .intersection(&feeds.values().cloned().collect())
        .map(|r| r.to_owned())
        .collect();

    if !roles_to_remove.is_empty() {
        msg.member(ctx)
            .await?
            .remove_roles(ctx, &roles_to_remove[..])
            .await?;
        msg.reply(ctx, "Role(s) removed").await?;
    } else {
        msg.reply(ctx, "No roles to remove").await?;
    }

    Ok(())
}
