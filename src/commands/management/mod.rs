use serenity::framework::standard::macros::group;

mod category;
mod rss;

use self::rss::RSS_GROUP;
use category::CATEGORY_COMMAND;

#[group]
#[prefix = "manage"]
#[allowed_roles("Staff")]
#[commands(category)]
#[sub_groups(Rss)]
struct Management;
