use crate::env::*;
use crate::State;

use serenity::async_trait;
use serenity::http::HttpError;
use serenity::model::channel::Reaction;
use serenity::model::guild::Member;
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

pub struct Handler;

#[allow(clippy::invisible_characters)]
const PT_WELCOME_MSG: &str = "**[PT]**\n\nBem vindo ao servidor de MEIC\n\nTemos algumas instruções que deves seguir para melhor interagires com os outros membros e para teres uma melhor experiência no Técnico\n**INSTRUÇÕES:**\nVai ao canal **#registration** e inscreve-te nas cadeiras que vais fazer reagindo às respetivas mensagens do bot com :raised_hand: (podes desinscrever-te ou reinscrever-te a qualquer momento removendo/readicionando a reação)\nIsto dar-te-á acesso ao role de cada cadeira (e ao respetivo canal), e esse role será taggado nos anúncios do Fénix dessa cadeira publicados em **#fenix** (podes dar mute à vontade, porque vais receber notificação quando o anúncio for de uma das tuas cadeiras)\n\nE temos também algumas regras para o bom funcionamento do servidor\n**REGRAS:**\n0. Tenta organizar-te, isto é, assuntos relacionados com o tema X são discutidos no canal X\n1. Muda o nick de forma a que os outros te consigam reconhecer, por exemplo, <primeiro nome> <apelido>, o Discord deixa-te ter um nick diferente para cada servidor, por isso não perdes o teu nick original\n2. Assuntos de cadeiras são discutidos nos canais das cadeiras, não faças spam no **#general** com pedidos de grupos/elementos para grupo\n3. Se há membros estrangeiros num canal, escreve em inglês (pelo menos nos canais das cadeiras)\n4. Usa menções a roles das cadeiras (@<nome cadeira>) em vez de (@​here/@​everyone) a não ser que precises mesmo de chamar a @​Staff ao barulho\n5. **Convites para o servidor** pede no **#gimme-invite-and-other-spam** (dá tag à @Staff)\n6. Usa o **#3rd-party-spam** para anúncios à comunidade\n\nDúvidas? Pergunta à vontade, alguém será capaz de te ajudar :wink:";

#[allow(clippy::invisible_characters)]
const EN_WELCOME_MSG: &str = "**[EN]**\n\nWelcome to the MEIC server\n\nWe have a few instructions that you should follow in order to better interact with the other members and for you to have a better experience in Técnico\n**INSTRUCTIONS:**\nGo to **#registration** and sign up for your courses by reacting to their respective messages with :raised_hand: (you can remove/add the reaction at any time to undo/redo this action).\nThis will give you access to the course's role (and respective channel). That role will be tagged in the course's Fénix announcements published in **#info-importante** (feel free to mute this channel as you will receive notifications from your course's announcements)\n\nWe also have a few rules to ensure the smooth operation of the server\n**RULES:**\n0. Try to organize your conversations. Topic X should be discussed inside channel X\n1. Change your nickname to something others can recognize (e.g. <firstname> <lastname>). Discord allows you to have a different nickname for server, so, your original nickname is kept in the other servers\n2. Course talks goes to course channels: don't spam **#general** with group paring stuff\n3. If there are non-portuguese speakers in a channel, please use english\n4. Use course role mentions (@<course name>) instead of the general ones (@​all/@​everyone/...), unless you really need @​Staff to hear about it\n5. **Invite links** ask in **#gimme-invite-and-other-spam** (tag @Staff)\n6. Use **#3rd-party-spam** for general announcements to everyone\n\nDoubts? Feel free to ask, someone might be able to help you :wink:
";

#[async_trait]
impl EventHandler for Handler {
    /// Handler for new members
    async fn guild_member_addition(&self, ctx: Context, _guild: GuildId, member: Member) {
        match member.user.dm(&ctx, |m| m.content(PT_WELCOME_MSG)).await {
            Ok(_) => {
                // Assume if one message is sent, the other one is too
                // At least, while serenity doesn't have a proper error to check if DMs are enabled
                let _ = member.user.dm(&ctx, |m| m.content(EN_WELCOME_MSG)).await;
            }
            Err(e) => match e {
                SerenityError::Http(e) => match *e {
                    HttpError::UnsuccessfulRequest(e) => {
                        if e.error.message == "Cannot send messages to this user" {
                            match ChannelId(
                                get_var(Variables::WelcomeChannel)
                                    .parse::<u64>()
                                    .expect("Welcome channel id is invalid!"),
                            )
                            .say(
                                &ctx,
                                MessageBuilder::new()
                                    .push("Hey ")
                                    .mention(&member)
                                    .push("!\nCheck channel ")
                                    .channel(ChannelId(
                                        get_var(Variables::RulesChannel)
                                            .parse::<u64>()
                                            .expect("Rules channel id is invalid!"),
                                    ))
                                    .build(),
                            )
                            .await
                            {
                                Ok(_) => (),
                                Err(e) => eprintln!("Unknown error on member join: {}", e),
                            };
                        } else {
                            eprintln!("Unknown error on member join: {:?}", e);
                        }
                    }
                    _ => eprintln!("Unknown error on member join: {}", e),
                },
                _ => eprintln!("Unknown error on member join: {}", e),
            },
        }
    }

    /// Handler for reactions added to messages
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        // Check if reaction is the one for reaction roles
        if reaction.emoji.unicode_eq(&get_var(Variables::ReactionRole)) {
            // Check if one of the special messages
            let lock = ctx.data.read().await;
            let role = lock
                .get::<State>()
                .expect("No state provided")
                .read()
                .await
                .get_messages()
                .get(&reaction.message_id.0)
                .map(|v| v.to_owned());

            if let Some(role) = role {
                // Intent for DMs is not enabled
                match match reaction
                    .guild_id
                    .unwrap()
                    .member(
                        &ctx,
                        match reaction.user(&ctx).await {
                            Ok(u) => u,
                            Err(e) => {
                                tracing::error!("Invalid user reacted: {}", e);
                                return;
                            }
                        },
                    )
                    .await
                {
                    Ok(m) => m,
                    Err(e) => {
                        tracing::error!("Invalid member reacted: {}", e);
                        return;
                    }
                }
                .add_role(&ctx, role)
                .await
                {
                    Ok(_) => (),
                    Err(e) => {
                        tracing::error!("Couldn't give role {}: {}", role, e);
                        let _ = cleanup_old_react_message(&ctx, reaction, e).await;
                        return;
                    }
                }
            }
            // Check if reaction is the one for pins
        } else if reaction.emoji.unicode_eq(&get_var(Variables::PinReaction))
            && match reaction
                .channel_id
                .reaction_users(&ctx, reaction.message_id, reaction.emoji, None, None)
                .await
            {
                Ok(u) => u,
                Err(e) => {
                    eprintln!("Couldn't get number of reactions: {}", e);

                    return;
                }
            }
            .len()
                >= get_var(Variables::PinMinReactions)
                    .parse::<usize>()
                    .expect("Minimum number of pins is not valid!")
        {
            match reaction.channel_id.pin(&ctx, reaction.message_id).await {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Couldn't pin message: {}", e);

                    return;
                }
            }
        }
    }

    /// Handler for reactions removed from messages
    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        // Check if reaction is the one for reaction roles
        if reaction.emoji.unicode_eq(&get_var(Variables::ReactionRole)) {
            // Check if one of the special messages
            let lock = ctx.data.read().await;
            let role = lock
                .get::<State>()
                .expect("No state provided")
                .read()
                .await
                .get_messages()
                .get(&reaction.message_id.0)
                .map(|v| v.to_owned());

            if let Some(role) = role {
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
                        let _ = cleanup_old_react_message(&ctx, reaction, e).await;
                        return;
                    }
                }
            }
        };
    }
}


async fn cleanup_old_react_message(ctx: &Context, reaction: Reaction, err: serenity::Error) -> eyre::Result<()> {
    if err.to_string() == "Unknown Role" {
        let m = reaction.message(&ctx.http).await?;
        m.delete(&ctx.http).await?;
    }

    Ok(())
}