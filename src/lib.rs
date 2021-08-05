mod commands;
pub(crate) mod env;
mod events;
pub(crate) mod model;

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
mod network;

use self::model::{update_all_feeds, Feed};

pub use self::commands::{after_hook, before_hook, COMMANDS_GROUP, HELP};
pub use self::env::*;
pub use self::events::Handler;

use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use serenity::prelude::TypeMapKey;
use serenity::CacheAndHttp;
use serenity::Result as SResult;

use tokio::sync::Mutex;
use tokio::time::sleep;

const TIME_BEFORE_UPDATE: u64 = 60; // 60 seconds

#[derive(Deserialize, Serialize)]
pub struct State {
    feeds: HashMap<String, Feed>,
    category: Option<u64>,
    messages: HashMap<u64, u64>, // <MessageId, RoleId>
}

impl State {
    /// Create an empty state
    pub fn new() -> Self {
        State {
            feeds: HashMap::new(),
            category: None,
            messages: HashMap::new(),
        }
    }

    pub(crate) fn get_messages(&self) -> &HashMap<u64, u64> {
        &self.messages
    }

    pub(crate) fn get_mut_messages(&mut self) -> &mut HashMap<u64, u64> {
        &mut self.messages
    }

    pub(crate) fn get_feeds(&self) -> &HashMap<String, Feed> {
        &self.feeds
    }

    pub(crate) fn get_mut_feeds(&mut self) -> &mut HashMap<String, Feed> {
        &mut self.feeds
    }

    pub(crate) fn set_category(&mut self, category: u64) {
        self.category = Some(category);
    }

    /// Save a list of feeds to a file
    pub fn save_to_file(file: &str, value: &State) -> Result<()> {
        fs::write(file, bincode::serialize(&value).unwrap())?;

        Ok(())
    }

    /// Load a list of feeds from a file
    pub fn load_from_file(file: &str) -> Result<State> {
        match bincode::deserialize(&fs::read(file)?) {
            Ok(val) => Ok(val),
            Err(e) => panic!("Invalid data!\nFeeds couldn't be loaded: {}", e),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

// So State can be included in the global state of the bot
impl TypeMapKey for State {
    type Value = Arc<Mutex<State>>;
}

/// Loop to update RSS feeds continuously
pub async fn rss(client: Arc<CacheAndHttp>, state: Arc<Mutex<State>>) -> SResult<()> {
    let time = Duration::new(
        get_var(Variables::RssSleep)
            .parse::<u64>()
            .expect("RSS sleep time is invalid!")
            * 60,
        0,
    );

    sleep(Duration::new(TIME_BEFORE_UPDATE, 0)).await;

    loop {
        update_all_feeds(&client, &mut *state.lock().await).await?;
        sleep(time).await;
    }
}
