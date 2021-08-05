use crate::env::*;

use rand::{thread_rng, Rng};

use serenity::client::Context;
use serenity::framework::standard::macros::group;
use serenity::model::channel::{
    ChannelType, GuildChannel, PermissionOverwrite, PermissionOverwriteType, ReactionType,
};
use serenity::model::guild::Role;
use serenity::model::id::{ChannelId, GuildId, MessageId, RoleId};
use serenity::model::permissions::Permissions;
use serenity::Error;

mod add;
mod import;
mod list;
mod mv;
mod rm;

use add::ADD_COMMAND;
use import::IMPORT_COMMAND;
use list::LIST_COMMAND;
use mv::MV_COMMAND;
use rm::RM_COMMAND;

const MAX_COLOR_VALUE: u64 = 16777215;

#[group]
#[prefix = "rss"]
#[commands(add, import, list, mv, rm)]
pub(crate) struct Rss;

/// Create a role if it doesn't exist and return it
///
/// If it exists, return it
pub(crate) async fn add_feed_role(
    ctx: &Context,
    guild: &GuildId,
    name: &str,
) -> Result<Role, Error> {
    let roles = guild.roles(ctx).await?;

    let role = roles.values().find(|r| r.name == name);

    if role.is_some() {
        return Ok(role.unwrap().to_owned());
    }

    Ok(guild
        .create_role(ctx, |r| {
            r.name(name)
                .mentionable(true)
                .colour(thread_rng().gen_range(0..MAX_COLOR_VALUE))
        })
        .await?)
}

/// Create a channel if it doesn't exist and return it
///
/// If it exists, return it
pub(crate) async fn add_feed_channel(
    ctx: &Context,
    guild: &GuildId,
    name: &str,
    role: RoleId,
    category: ChannelId,
) -> Result<GuildChannel, Error> {
    let channels = guild.channels(ctx).await?;

    let channel = channels.values().find(|c| c.name == name.to_lowercase());

    if channel.is_some() {
        return Ok(channel.unwrap().to_owned());
    }

    // The everyone role id is equal to the guild id
    let everyone_permissions = PermissionOverwrite {
        allow: Permissions::empty(),
        deny: Permissions::READ_MESSAGES,
        kind: PermissionOverwriteType::Role(RoleId(guild.0)),
    };

    let role_permissions = PermissionOverwrite {
        allow: Permissions::READ_MESSAGES,
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Role(role),
    };

    let delegate_permissions = PermissionOverwrite {
        allow: Permissions::READ_MESSAGES,
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Role(RoleId(
            get_var(Variables::DelegateRole)
                .parse::<u64>()
                .expect("Delegate role id is not valid!"),
        )),
    };

    Ok(guild
        .create_channel(ctx, |c| {
            c.name(name)
                .kind(ChannelType::Text)
                .permissions(vec![
                    role_permissions,
                    delegate_permissions,
                    everyone_permissions,
                ])
                .category(category)
        })
        .await?)
}

/// Add feed reaction role message
pub(crate) async fn add_feed_message(
    ctx: &Context,
    name: &str,
    role: RoleId,
    channel: ChannelId,
) -> Result<MessageId, Error> {
    let reaction = get_var(Variables::ReactionRole);

    Ok(ChannelId(
                    get_var(Variables::ReactionRolesChannel)
                        .parse::<u64>()
                        .expect("React roles' channel id is not valid!"),
                )
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.title(format!("[{}] Cadeira disponível / Course available", name));

                        // Can't use const's as format strings
                        e.description(format!("**[PT]**\n\nSe vai fazer a cadeira **{}** reage com {} para teres acesso ao role {}, ao canal {} e receberes notificações de anúncios\nPara remover tudo isto só precisas de remover a reação\n\n**[EN]**\n\nIf you are enrolling in **{}** react with {} to get access to the role {}, the channel {} and to receive announcements' notifications\nTo quit all of this, just remove the reaction", name, reaction, role, channel, name, reaction, role, channel));

                        e
                    });
                    m.reactions(vec![ReactionType::Unicode(reaction)]);

                    m
                })
                .await?.id)
}

/// Remove feed reaction role message
pub(crate) async fn rm_feed_message(ctx: &Context, msg: MessageId) -> Result<(), Error> {
    ChannelId(
        get_var(Variables::ReactionRolesChannel)
            .parse::<u64>()
            .expect("React roles' channel id is not valid"),
    )
    .delete_message(ctx, msg)
    .await?;

    Ok(())
}

/// Rename role
pub(crate) async fn mv_role(
    ctx: &Context,
    guild: GuildId,
    role: RoleId,
    name: String,
) -> Result<(), Error> {
    guild.edit_role(ctx, role, |r| r.name(name)).await?;

    Ok(())
}

/// Rename channel
pub(crate) async fn mv_channel(
    ctx: &Context,
    channel: ChannelId,
    name: String,
) -> Result<(), Error> {
    channel
        .to_channel(&ctx)
        .await?
        .guild()
        .unwrap()
        .edit(ctx, |c| c.name(name))
        .await?;

    Ok(())
}
