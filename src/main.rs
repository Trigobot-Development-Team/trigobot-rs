use serenity::framework::StandardFramework;
use serenity::prelude::*;

use trigobot::commands::{after_hook, before_hook, COMMANDS_GROUP};
use trigobot::env::*;

#[tokio::main]
async fn main() {
    // Check if all variables are correctly defined
    test_env();

    let token = get_var(Variables::DiscordToken);

    let mut client = Client::builder(&token)
        .framework(
            StandardFramework::new()
                .configure(|c| c.prefix(&get_var(Variables::CommandPrefix)))
                .before(before_hook)
                .after(after_hook)
                .group(&COMMANDS_GROUP),
        )
        .await
        .expect("Couldn't create client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}
