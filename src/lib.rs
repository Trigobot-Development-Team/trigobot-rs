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
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use serenity::model::id::RoleId;
use serenity::prelude::TypeMapKey;
use serenity::CacheAndHttp;

use tokio::sync::RwLock;
use tokio::time::sleep;

use eyre::{Result, WrapErr};

const TIME_BEFORE_UPDATE: u64 = 60; // 60 seconds

#[derive(Default, Deserialize, Serialize)]
pub struct State {
    feeds: HashMap<String, Feed>,
    category: Option<u64>,
    messages: HashMap<u64, u64>, // <MessageId, RoleId>
}

impl State {
    /// Create an empty state
    pub fn new() -> Self {
        State::default()
    }

    pub(crate) fn get_messages(&self) -> &HashMap<u64, u64> {
        &self.messages
    }

    pub(crate) fn get_mut_messages(&mut self) -> &mut HashMap<u64, u64> {
        &mut self.messages
    }

    pub(crate) fn get_feeds_simple(&self) -> HashMap<String, RoleId> {
        self.feeds
            .iter()
            .map(|(name, feed)| (name.to_string(), feed.get_role()))
            .collect()
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
    pub fn save_to_file(file_path: impl AsRef<Path>, value: &State) -> Result<()> {
        use std::io::Write;

        let state_bytes = bincode::serialize(&value)
            .wrap_err("failed to serialize state")?;

        // We use a tempfile to make the state saving crash-safe
        // Save it in the same directory as the state file to prevent moving errors
        let tmpdir = file_path
            .as_ref()
            .parent()
            .unwrap_or_else(|| Path::new("."));
        let mut tmp = tempfile::NamedTempFile::new_in(tmpdir)?;
        tmp.write_all(&state_bytes)?;

        // atomically moves the tempfile to its final location
        tmp.persist(file_path)?;

        Ok(())
    }

    /// Load a list of feeds from a file
    pub fn load_from_file(file: impl AsRef<Path>) -> Result<State> {
        match bincode::deserialize(&fs::read(file)?) {
            Ok(val) => Ok(val),
            Err(e) => panic!("Invalid data!\nFeeds couldn't be loaded: {}", e),
        }
    }
}

// So State can be included in the global state of the bot
impl TypeMapKey for State {
    type Value = Arc<RwLock<State>>;
}

/// Loop to update RSS feeds continuously
pub async fn rss(client: Arc<CacheAndHttp>, state: Arc<RwLock<State>>) -> ! {
    let time = Duration::new(
        get_var(Variables::RssSleep)
            .parse::<u64>()
            .expect("RSS sleep time is invalid!")
            * 60,
        0,
    );

    sleep(Duration::new(TIME_BEFORE_UPDATE, 0)).await;

    loop {
        if let Err(_) = update_all_feeds(&client, &mut *state.write().await).await {
            // don't crash
        }
        sleep(time).await;
    }
}
