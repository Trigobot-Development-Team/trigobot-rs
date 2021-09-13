use serenity::framework::standard::macros::group;

mod add;
mod clear;
mod rm;

use add::ADD_COMMAND;
use clear::CLEAR_COMMAND;
use rm::RM_COMMAND;

#[group]
#[prefix("roles")]
#[summary("Commands to manage user roles")]
#[commands(add, clear, rm)]
struct Roles;
