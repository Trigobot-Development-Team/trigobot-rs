use serenity::framework::standard::macros::group;

mod anunciar;

use anunciar::ANUNCIAR_COMMAND;

#[group]
#[commands(anunciar)]
pub struct Commands;
