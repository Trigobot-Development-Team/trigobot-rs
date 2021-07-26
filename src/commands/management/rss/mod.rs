use serenity::framework::standard::macros::group;

mod add;
mod rm;

use add::ADD_COMMAND;
use rm::RM_COMMAND;

#[group]
#[prefix = "rss"]
#[commands(add, rm)]
pub(crate) struct Rss;
