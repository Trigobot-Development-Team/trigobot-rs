use crate::env::*;
use crate::State;

use serenity::async_trait;
use serenity::model::channel::Reaction;
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /// Handler for reactions added to messages
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        // Check if reaction is the one for reaction roles
        if reaction.emoji.unicode_eq(&get_var(Variables::ReactionRole)) {
            // Check if one of the special messages
            if let Some(role) = {
                let lock = ctx.data.read().await;

                match lock
                    .get::<State>()
                    .expect("No state provided")
                    .get_messages()
                    .get(&reaction.message_id.0)
                {
                    None => None,
                    Some(r) => Some(r.to_owned()),
                }
            } {
                // Intent for DMs is not enabled
                match match reaction
                    .guild_id
                    .unwrap()
                    .member(
                        &ctx,
                        match reaction.user(&ctx).await {
                            Ok(u) => u,
                            Err(e) => {
                                eprintln!("Invalid user reacted: {}", e);
                                return;
                            }
                        },
                    )
                    .await
                {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("Invalid member reacted: {}", e);
                        return;
                    }
                }
                .add_role(&ctx, role)
                .await
                {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Couldn't give role {}: {}", role, e);
                        return;
                    }
                }
            }
        };

        ()
    }

    /// Handler for reactions removed from messages
    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        // Check if reaction is the one for reaction roles
        if reaction.emoji.unicode_eq(&get_var(Variables::ReactionRole)) {
            // Check if one of the special messages
            if let Some(role) = {
                let lock = ctx.data.read().await;

                match lock
                    .get::<State>()
                    .expect("No state provided")
                    .get_messages()
                    .get(&reaction.message_id.0)
                {
                    None => None,
                    Some(r) => Some(r.to_owned()),
                }
            } {
                // Intent for DMs is not enabled
                match match reaction
                    .guild_id
                    .unwrap()
                    .member(
                        &ctx,
                        match reaction.user(&ctx).await {
                            Ok(u) => u,
                            Err(e) => {
                                eprintln!("Invalid user reacted: {}", e);
                                return;
                            }
                        },
                    )
                    .await
                {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("Invalid member reacted: {}", e);
                        return;
                    }
                }
                .remove_role(&ctx, role)
                .await
                {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Couldn't give role {}: {}", role, e);
                        return;
                    }
                }
            }
        };

        ()
    }
}
