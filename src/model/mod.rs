mod cache;
mod feed;
mod message;

pub(crate) use cache::*;
pub use feed::*;
pub(crate) use message::*;

use crate::env::*;
use crate::State;

use futures::future::join_all;

use serenity::http::CacheHttp;
use serenity::model::id::{ChannelId, RoleId};
use serenity::utils::{Colour, MessageBuilder};
use eyre::{Result, WrapErr, eyre};

const EMBED_MAX_DESC: usize = 4096;

/// Update **ALL** feeds
pub(crate) async fn update_all_feeds<T: CacheHttp>(ctx: T, state: &mut State) -> Result<()> {
    join_all(
        state
            .get_mut_feeds()
            .values_mut()
            .into_iter()
            .map(|f| update_feed(&ctx, f))
            .collect::<Vec<_>>(),
    )
    .await;

    State::save_to_file(&get_var(Variables::StateFile), state)
}

#[tracing::instrument(skip(ctx))]
async fn update_feed<T: CacheHttp>(ctx: &T, feed: &mut Feed) -> Result<()> {
    let announcements_channel = ChannelId(
        get_var(Variables::AnnouncementsChannel)
            .parse::<u64>()
            .wrap_err("Invalid announcement channel ID")?,
    );

    let role = feed.get_role();

    let color = role
        .to_role_cached(&ctx.cache().ok_or(eyre!("no cache provided"))?)
        .await
        .ok_or(eyre!("failed to fetch role for feed"))?
        .colour;

    let update_ts = feed.get_update();

    let messages = feed.update().await;

    for m in messages {
        let copy = m.clone();

        // Check if announcement is too old, needs to be updated or is new
        let id = if m.timestamp <= update_ts {
            if m.link.is_some() {
                if let Some(old_id) = feed.check_changed(&m) {
                    edit_announcement(
                        ctx,
                        announcements_channel,
                        old_id,
                        feed.get_name(),
                        m,
                        role,
                        color,
                    )
                    .await?
                } else {
                    continue;
                }
            } else {
                continue;
            }
        } else {
            publish_announcement(ctx, announcements_channel, feed.get_name(), m, role, color)
                .await?
        };

        feed.cache(id, copy);
    }

    Ok(())
}

async fn publish_announcement<T: CacheHttp>(
    ctx: &T,
    announcements_channel: ChannelId,
    name: String,
    message: Message,
    role: RoleId,
    color: Colour,
) -> Result<u64> {
    Ok(announcements_channel
        .send_message(&ctx.http(), |a| {
            a.content(
                MessageBuilder::new()
                    .mention(&role)
                    .push_line("")
                    .push_bold_line_safe(message.title.clone())
                    .build(),
            );

            a.embed(|e| {
                e.title(format!("[{}] {}", name, message.title));

                e.author(|a| {
                    a.icon_url(get_var(Variables::AnnouncementIcon));
                    a.name(message.author.clone());

                    a
                });

                if message.content.len() > EMBED_MAX_DESC {
                    e.description(prune_msg(&message.content, EMBED_MAX_DESC - 5) + "(...)");
                } else {
                    e.description(message.content);
                }

                e.color(color);

                if let Some(l) = message.link {
                    e.field(
                        "Original Announcement",
                        format!("[Click here]({})", l),
                        false,
                    );
                }

                e
            });

            a
        })
        .await?
        .id
        .0)
}

async fn edit_announcement<T: CacheHttp>(
    ctx: &T,
    announcements_channel: ChannelId,
    old_id: u64,
    name: String,
    message: Message,
    role: RoleId,
    color: Colour,
) -> Result<u64> {
    // Get old message
    let old_msg = announcements_channel.message(&ctx.http(), old_id).await?;

    // Edit old announcement
    announcements_channel
        .edit_message(&ctx.http(), old_id, |m| {
            m.embed(|e| {
                e.title(format!("[{}] {}", name, message.title));

                e.author(|a| {
                    a.icon_url(get_var(Variables::AnnouncementIcon));
                    a.name(message.author.clone());

                    a
                });

                e.description(
                    "**THIS ANNOUNCEMENT HAS BEEN UPDATED**\n~~".to_owned()
                        + &prune_msg(
                            match &old_msg.embeds[0].description {
                                None => "",
                                Some(val) => val,
                            },
                            EMBED_MAX_DESC - 43,
                        )
                        + "~~",
                );

                e.color(color);

                if let Some(l) = &message.link {
                    e.field(
                        "Original Announcement",
                        format!("[Click here]({})", l),
                        false,
                    );
                }

                e
            })
        })
        .await?;

    publish_announcement(ctx, announcements_channel, name, message, role, color).await
}

fn prune_msg(msg: &str, len: usize) -> String {
    msg.chars()
        .collect::<Vec<char>>()
        .chunks(len)
        .next()
        .unwrap()
        .iter()
        .collect()
}
