use serenity::framework::StandardFramework;
use serenity::prelude::*;

use trigobot::commands::{after_hook, before_hook, COMMANDS_GROUP, HELP};
use trigobot::env::*;
use trigobot::State;

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
                .group(&COMMANDS_GROUP)
                .help(&HELP),
        )
        .await
        .expect("Couldn't create client");

    {
        let mut data = client.data.write().await;

        data.insert::<State>(
            match State::load_from_file(&get_var(Variables::FeedsFile)) {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Couldn't load feeds from file: {}\n", e);

                    State::new()
                }
            },
        )
    }

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}
