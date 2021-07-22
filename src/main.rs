use std::collections::HashMap;
use std::sync::Arc;

use serenity::framework::StandardFramework;
use serenity::prelude::*;

use trigobot::commands::{after_hook, before_hook, COMMANDS_GROUP};
use trigobot::env::*;
use trigobot::Feed;

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

    {
        let mut data = client.data.write().await;

        data.insert::<Feed>(Arc::new(
            match Feed::load_from_file(&get_var(Variables::FeedsFile)) {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Couldn't load feeds from file: {}\n", e);

                    HashMap::new()
                }
            },
        ))
    }

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}
