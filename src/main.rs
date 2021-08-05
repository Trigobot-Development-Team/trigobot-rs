use std::sync::Arc;

use serenity::client::bridge::gateway::GatewayIntents;
use serenity::framework::StandardFramework;
use serenity::prelude::*;

use tokio::sync::Mutex;
use tokio::try_join;

use trigobot::*;

#[tokio::main]
async fn main() {
    // Check if all variables are correctly defined
    test_env();

    let token = get_var(Variables::DiscordToken);

    let mut client = Client::builder(&token)
        .intents(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MESSAGE_REACTIONS
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

    let state = Arc::new(Mutex::new(
        match State::load_from_file(&get_var(Variables::StateFile)) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Couldn't load feeds from file: {}\n", e);

                State::new()
            }
        },
    ));

    {
        let mut data = client.data.write().await;

        data.insert::<State>(Arc::clone(&state));
    }

    let http_client = Arc::clone(&client.cache_and_http);

    match try_join!(client.start(), rss(http_client, state)) {
        Ok(_) => (),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}
