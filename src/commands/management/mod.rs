use crate::env::*;

use rand::{thread_rng, Rng};

use serenity::client::Context;
use serenity::framework::standard::macros::group;
use serenity::model::channel::{
    ChannelType, GuildChannel, PermissionOverwrite, PermissionOverwriteType,
};
use serenity::model::guild::Role;
use serenity::model::id::{ChannelId, GuildId, RoleId};
use serenity::model::permissions::Permissions;
use serenity::Error;

mod category;
mod rss;

use self::rss::RSS_GROUP;
use category::CATEGORY_COMMAND;

const MAX_COLOR_VALUE: u64 = 16777215;

#[group]
#[prefix = "manage"]
#[allowed_roles("Staff")]
#[commands(category)]
#[sub_groups(Rss)]
struct Management;

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

    let channel = channels.values().find(|c| c.name == name);

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
