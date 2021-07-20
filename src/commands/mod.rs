use serenity::framework::standard::macros::group;

mod announce;
mod say;

use announce::ANNOUNCE_COMMAND;
use say::SAY_COMMAND;

#[group]
#[commands(announce, say)]
pub struct Commands;
