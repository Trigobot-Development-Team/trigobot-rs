use std::env::var as ENV;

use dotenv::dotenv;

use serenity::prelude::*;

const VAR_DISCORD_TOKEN: &str = "DISCORD_TOKEN";

#[tokio::main]
async fn main() {
    // Load .env file vars
    dotenv().ok();

    let token = ENV(VAR_DISCORD_TOKEN).expect(&format!(
        "No Discord token found!\nSet env var {} with the token",
        VAR_DISCORD_TOKEN
    ));

    let mut client = Client::builder(&token)
        .await
        .expect("Couldn't create client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}
