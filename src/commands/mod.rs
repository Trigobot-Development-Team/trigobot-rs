use serenity::client::Context;
use serenity::framework::standard::macros::{group, hook};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

mod announce;
mod email;
mod say;

use announce::ANNOUNCE_COMMAND;
use email::EMAIL_COMMAND;
use say::SAY_COMMAND;

#[group]
#[commands(announce, email, say)]
struct Commands;

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
