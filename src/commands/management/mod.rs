use serenity::framework::standard::macros::group;

mod category;

use category::CATEGORY_COMMAND;

#[group]
#[prefix = "manage"]
#[allowed_roles("Staff")]
#[commands(category)]
struct Management;
