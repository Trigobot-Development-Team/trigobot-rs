use std::sync::Arc;

use futures::FutureExt;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::framework::StandardFramework;
use serenity::prelude::*;

use tokio::sync::RwLock;
use tokio::try_join;

use trigobot::*;

#[tokio::main]
async fn main() {
    // Check if all variables are correctly defined
    load_env();

    let subscriber = tracing_subscriber::fmt().finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to setup console logging");

    let token = get_var(Variables::DiscordToken);

    let mut client = Client::builder(&token)
        .intents(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MESSAGE_REACTIONS
                | GatewayIntents::GUILD_PRESENCES
                | GatewayIntents::DIRECT_MESSAGES,
        )
        .event_handler(Handler)
        .framework(
            StandardFramework::new()
                .configure(|c| c.prefix(&get_var(Variables::CommandPrefix)))
                .before(before_hook)
                .after(after_hook)
                .group(&COMMANDS_GROUP)
                .help(&HELP),
        )
        .await
        .expect("Couldn't create client");

    let state = Arc::new(RwLock::new(
        match State::load_from_file(get_var(Variables::StateFile)) {
            Ok(val) => val,
            Err(error) => {
                tracing::error!(%error, "Couldn't load feeds from file");

                State::new()
            }
        },
    ));

    {
        let mut data = client.data.write().await;

        data.insert::<State>(Arc::clone(&state));
    }

    let http_client = Arc::clone(&client.cache_and_http);

    match try_join!(
        client.start(),
        rss(http_client, state).map(|_| Ok(())),
        check_env()
    ) {
        Ok(_) => (),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}
