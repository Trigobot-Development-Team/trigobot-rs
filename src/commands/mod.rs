use std::collections::HashSet;

use serenity::client::Context;
use serenity::framework::standard::macros::{group, help, hook};
use serenity::framework::standard::{
    help_commands, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::channel::Message;
use serenity::model::id::UserId;

mod announce;
mod dump;
mod email;
mod management;
mod rss;
mod say;
mod traps;
mod undo;

use self::rss::RSS_COMMAND;
use announce::ANNOUNCE_COMMAND;
use dump::DUMP_COMMAND;
use email::EMAIL_COMMAND;
use management::MANAGEMENT_GROUP;
use say::SAY_COMMAND;
use traps::TRAPS_COMMAND;
use undo::UNDO_COMMAND;

#[group]
#[commands(announce, dump, email, rss, say, traps, undo)]
#[sub_groups(Management)]
struct Commands;

#[help]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
pub async fn before_hook(_ctx: &Context, msg: &Message, _command: &str) -> bool {
    println!(
        "[{}] on <#{}>: {}",
        msg.author.name,
        if msg.is_private() {
            "DM".to_string()
        } else {
            msg.channel_id.to_string()
        },
        msg.content
    );

    true
}

#[hook]
pub async fn after_hook(_ctx: &Context, _msg: &Message, command: &str, res: CommandResult) {
    match res {
        Ok(_) => (),
        Err(e) => eprintln!("Command '{}' returned error {:?}", command, e),
    }
}
