use serenity::framework::standard::macros::group;

mod announce;
mod email;
mod say;

use announce::ANNOUNCE_COMMAND;
use email::EMAIL_COMMAND;
use say::SAY_COMMAND;

#[group]
#[commands(announce, email, say)]
struct Commands;
