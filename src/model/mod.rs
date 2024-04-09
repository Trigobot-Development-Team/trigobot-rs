mod cache;
mod feed;
mod message;

pub(crate) use cache::*;
pub use feed::*;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use futures::TryFutureExt;
pub(crate) use message::*;

use crate::env::*;
use crate::State;

use eyre::{eyre, WrapErr};
use serenity::http::CacheHttp;
use serenity::model::id::{ChannelId, RoleId};
use serenity::utils::{Colour, MessageBuilder};

const EMBED_MAX_DESC: usize = 4096;

/// Update **ALL** feeds
#[tracing::instrument(skip_all)]
pub(crate) async fn update_all_feeds<T: CacheHttp>(
    ctx: T,
    state: &mut State,
) -> Result<(), Vec<(String, eyre::Report)>> {
    let feed_updates = state
        .get_mut_feeds()
        .values_mut()
        .map(|feed| {
            let name = feed.get_name();
            update_feed(&ctx, feed).map_err(|e| (name, e))
        })
        .collect::<FuturesUnordered<_>>();

    let mut errors = feed_updates
        .filter_map(|res| async move { res.err() })
        .collect::<Vec<_>>()
        .await;

    if let Err(error) = State::save_to_file(get_var(Variables::StateFile), state) {
        tracing::error!(%error, "failed to save state after feed updates");
        errors.push((
            "(none)".to_owned(),
            error.wrap_err("failed to save state after feed updates"),
        ));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[allow(clippy::blocks_in_conditions)]
#[tracing::instrument(skip(ctx), err)]
async fn update_feed<T: CacheHttp>(ctx: &T, feed: &mut Feed) -> eyre::Result<()> {
    let announcements_channel = ChannelId(
        get_var(Variables::AnnouncementsChannel)
            .parse::<u64>()
            .trace_wrap_err("(announcement) channel id must be a valid u64")?,
    );

    let role = feed.get_role();

    let color = role
        .to_role_cached(&ctx.cache().ok_or(eyre!("no cache provided")).trace_err()?)
        .await
        .ok_or(eyre!("failed to fetch role"))
        .trace_err()?
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
                    .await
                    .trace_wrap_err("error editing announcement")?
                } else {
                    continue;
                }
            } else {
                continue;
            }
        } else {
            publish_announcement(ctx, announcements_channel, feed.get_name(), m, role, color)
                .await
                .trace_wrap_err("error publishing announcement")?
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
) -> eyre::Result<u64> {
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
) -> eyre::Result<u64> {
    // Get old message
    let old_msg = announcements_channel
        .message(&ctx.http(), old_id)
        .await
        .trace_wrap_err("error fetching previous announcement")?;

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

trait ResultTraceWrapErrorExt<T, E> {
    fn trace_wrap_err(self, msg: &'static str) -> eyre::Result<T>;
    fn trace_err(self) -> Result<T, E>;
}

impl<T, E> ResultTraceWrapErrorExt<T, E> for Result<T, E>
where
    E: std::fmt::Display,
    Result<T, E>: eyre::WrapErr<T, E>,
{
    fn trace_wrap_err(self, msg: &'static str) -> Result<T, eyre::Report> {
        match self {
            Ok(v) => Ok(v),
            Err(error) => {
                tracing::error!(%error, msg);
                Err(error).wrap_err(msg)
            }
        }
    }

    fn trace_err(self) -> Result<T, E> {
        match self {
            Ok(v) => Ok(v),
            Err(error) => {
                tracing::error!(%error);
                Err(error)
            }
        }
    }
}
