use crate::env::*;
use crate::model::Feed;
use crate::State;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::utils::MessageBuilder;
use serenity::Error;

#[command]
async fn rss(ctx: &Context, msg: &Message) -> CommandResult {
    let channel = msg.channel_id;

    // Can't delete messages in DMs
    if !msg.is_private() {
        match msg.delete(&ctx).await {
            Ok(_) => (),
            Err(_) => eprintln!(
                "[RSS] Couldn't delete message from {} on channel {}",
                msg.author.name, msg.channel_id
            ),
        };
    }

    let announcements_channel = ChannelId(
        get_var(Variables::AnnouncementsChannel)
            .parse::<u64>()
            .expect("Announcement channel id is not valid!"),
    );

    {
        let mut lock = ctx.data.write().await;

        let state = lock.get_mut::<State>().expect("No state provided");

        let _ = state.get_mut_feeds().values_mut().into_iter().map(
            |f: &mut Feed| -> Result<(), Error> {
                async {
                    for m in f.update().await {
                        announcements_channel
                            .send_message(ctx, |a| {
                                a.content(
                                    MessageBuilder::new()
                                        .push_bold_line_safe(m.title.clone())
                                        .build(),
                                );

                                a.embed(|e| {
                                    e.title(m.title.clone());

                                    e.author(|a| {
                                        a.icon_url(get_var(Variables::AnnouncementIcon));
                                        a.name(m.author.clone());

                                        a
                                    });

                                    e.description(m.content);

                                    if let Some(l) = m.link {
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
                            .await?;
                    }

                    return Ok::<(), Error>(());
                };

                Ok(())
            },
        );
    }

    channel
        .say(
            ctx,
            MessageBuilder::new()
                .push("Feeds atualizados com sucesso")
                .build(),
        )
        .await?;

    Ok(())
}
