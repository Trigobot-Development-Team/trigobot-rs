use crate::State;

use std::collections::HashSet;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::id::RoleId;

#[command]
#[description("Add roles to the user")]
#[usage("<roles>")]
#[example("IAC IEI")]
#[min_args(1)]
#[only_in(guilds)]
async fn add(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let roles: HashSet<String> = args.rest().split(' ').map(|s| s.to_string()).collect();

    let feeds = {
        let lock = ctx.data.read().await;

        let state = lock.get::<State>().expect("No state provided").read().await;

        state.get_feeds_simple()
    };

    let roles_to_add: HashSet<RoleId> = roles
        .intersection(&feeds.keys().cloned().collect())
        .map(|n| feeds.get(n).unwrap().to_owned())
        .collect();

    if !roles_to_add.is_empty() {
        let mut member = msg.member(ctx).await?;
        let final_roles: Vec<RoleId> = (&roles_to_add
            - &(member.roles.clone().drain(..).collect::<HashSet<RoleId>>()))
            .iter()
            .map(|r| r.to_owned())
            .collect();

        if !final_roles.is_empty() {
            member.add_roles(ctx, &final_roles[..]).await?;
            msg.reply(ctx, "Role(s) added").await?;
        } else {
            msg.reply(ctx, "You already have all those roles").await?;
        }
    } else {
        msg.reply(ctx, "No valid roles detected").await?;
    }

    Ok(())
}
