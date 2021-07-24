use serenity::framework::standard::macros::group;

mod add;

use add::ADD_COMMAND;

#[group]
#[prefix = "rss"]
#[commands(add)]
pub(crate) struct Rss;
