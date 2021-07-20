use std::env::var as ENV;

use dotenv::dotenv;

use serenity::framework::StandardFramework;
use serenity::prelude::*;

use trigobot::commands::COMMANDS_GROUP;

const VAR_DISCORD_TOKEN: &str = "DISCORD_TOKEN";
const VAR_COMMAND_PREFIX: &str = "COMMAND_PREFIX";

#[tokio::main]
async fn main() {
    // Load .env file vars
    dotenv().ok();

    let token = ENV(VAR_DISCORD_TOKEN).expect(&format!(
        "No Discord token found!\nSet env var {} with the token",
        VAR_DISCORD_TOKEN
    ));

    let mut client = Client::builder(&token)
        .framework(
            StandardFramework::new()
                .configure(|c| {
                    c.prefix(&ENV(VAR_COMMAND_PREFIX).expect(&format!(
                        "No command prefix defined!\nSet env var {} with the token",
                        VAR_COMMAND_PREFIX
                    )))
                })
                .group(&COMMANDS_GROUP),
        )
        .await
        .expect("Couldn't create client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}
