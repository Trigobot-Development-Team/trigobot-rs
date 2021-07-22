use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use bincode;

use chrono::DateTime;

use regex::Regex;

use rss::Channel;

use serde::{Deserialize, Serialize};

use serenity::model::channel::GuildChannel;
use serenity::model::guild::Role;
use serenity::prelude::TypeMapKey;

/// Stores all information regarding a feed
///
/// Namely:
/// + its name
/// + its link
/// + its role on the server
/// + its channel on the server
/// + the last time it was updated
#[derive(Deserialize, Serialize)]
pub struct Feed {
    name: String,
    link: String,
    role: Option<Role>,            // TODO: CHANGE
    channel: Option<GuildChannel>, // TODO: CHANGE
    updated: u64,
}

impl Feed {
    /// Creates a new feed to track
    ///
    /// Channel and Role need to be created *a priori*
    ///
    /// Link needs to be checked *a priori*
    pub fn new(
        name: String,
        link: String,
        role: Option<Role>,            // TODO: CHANGE
        channel: Option<GuildChannel>, // TODO: CHANGE
        updated: Option<u64>,
    ) -> Self {
        Feed {
            name,
            link,
            role,
            channel,
            updated: if updated.is_some() {
                updated.unwrap()
            } else {
                0
            },
        }
    }

    /// Check if link is RSS (supported by crate `rss`)
    pub async fn test_link(link: &str) -> bool {
        let response = reqwest::get(link).await;

        if response.is_err() {
            return false;
        }

        let response = response.unwrap().bytes().await;

        if response.is_err() {
            return false;
        }

        Channel::read_from(&response.unwrap()[..]).is_ok()
    }

    /// Retrieve new messages from the feed (if available)
    pub(crate) async fn update(&mut self) -> Vec<Message> {
        // Will stop if feed cannot be reached
        let response = match reqwest::get(self.link.clone()).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[RSS UPDATE] Couldn't fetch feed {}: {}", self.name, e);

                return vec![];
            }
        };

        let response = match response.bytes().await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[RSS UPDATE] Couldn't fetch feed {}: {}", self.name, e);

                return vec![];
            }
        };

        let messages = match Channel::read_from(&response[..]) {
            Ok(c) => c.items,
            Err(e) => {
                eprintln!("[RSS UPDATE] Couldn't parse feed {}: {}", self.name, e);

                return vec![];
            }
        };

        let reg_author = Regex::new(r"\((.*)\)").unwrap();

        // Iterate from the newer to the older messages
        let mut result: Vec<Message> = messages
            .iter()
            .filter_map(|m| {
                if m.pub_date.is_some() {
                    let ts = match DateTime::parse_from_rfc2822(&m.pub_date.clone().unwrap()) {
                        Ok(d) => d,
                        Err(e) => {
                            eprintln!(
                                "[RSS UPDATE] Couldn't parse date on feed {}: {}",
                                self.name, e
                            );

                            return None;
                        }
                    }
                    .timestamp() as u64;

                    // Select most recent messages only
                    if ts > self.updated {
                        Some(Message::new(
                            match &m.author {
                                Some(a) => match reg_author.captures(&a) {
                                    Some(c) => match c.get(1) {
                                        Some(a) => a.as_str().to_owned(),
                                        None => "Teacher Trigobot".to_owned(),
                                    },
                                    None => "Teacher Trigobot".to_owned(),
                                },
                                None => "Teacher Trigobot".to_owned(),
                            },
                            html2md::parse_html(
                                &m.title
                                    .clone()
                                    .unwrap_or("Trigobot for President".to_owned()),
                            ),
                            html2md::parse_html(
                                &m.description
                                    .clone()
                                    .unwrap_or("I am inevitable".to_owned()),
                            ),
                            m.link.clone(),
                            ts,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        self.updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards :O")
            .as_secs();

        result.sort();

        result
    }

    /// Save a list of feeds to a file
    pub fn save_to_file(file: &str, value: &HashMap<String, Feed>) -> Result<()> {
        fs::write(file, bincode::serialize(&value).unwrap())?;

        Ok(())
    }

    /// Load a list of feeds from a file
    pub fn load_from_file(file: &str) -> Result<HashMap<String, Feed>> {
        match bincode::deserialize(&fs::read(file)?) {
            Ok(val) => Ok(val),
            Err(e) => panic!("Invalid data!\nFeeds couldn't be loaded: {}", e),
        }
    }
}

// For Serenity
impl TypeMapKey for Feed {
    type Value = Arc<HashMap<String, Feed>>;
}

/// Stores all relevant information to create a Discord message
#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) struct Message {
    pub author: String,
    pub title: String,
    pub content: String,
    pub link: Option<String>,
    timestamp: u64,
}

impl Message {
    pub fn new(
        author: String,
        title: String,
        content: String,
        link: Option<String>,
        timestamp: u64,
    ) -> Self {
        Message {
            author,
            title,
            content,
            link,
            timestamp,
        }
    }
}

impl Eq for Message {}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}
