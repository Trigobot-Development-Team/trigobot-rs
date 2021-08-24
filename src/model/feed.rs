use super::{Cache, Message};
use crate::env::*;

use chrono::DateTime;

use md5;

use regex::Regex;

use rss::Channel;

use serde::{Deserialize, Serialize};

use serenity::model::id::{ChannelId, MessageId, RoleId};

/// Stores all information regarding a feed
///
/// Namely:
/// + its name
/// + its link
/// + its role on the server
/// + its channel on the server
/// + the message to give the role
/// + the last time it was updated
#[derive(Deserialize, Serialize)]
pub struct Feed {
    name: String,
    link: String,
    role: u64,
    cache: Cache<String, MessageInfo>, // Key is the link and value is tuple (msg_id, hash)
    channel: u64,
    message: u64,
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
        role: u64,
        channel: u64,
        message: u64,
        updated: Option<u64>,
    ) -> Self {
        Feed {
            name,
            link,
            role,
            cache: Cache::new(
                get_var(Variables::CacheEntries)
                    .parse::<usize>()
                    .expect("Number of cache entries is invalid!"),
            ),
            channel,
            updated: if let Some(val) = updated { val } else { 0 },
            message,
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

    pub(crate) fn get_name(&self) -> String {
        self.name.clone()
    }

    pub(crate) fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub(crate) fn get_role(&self) -> RoleId {
        RoleId(self.role)
    }

    pub(crate) fn get_channel(&self) -> ChannelId {
        ChannelId(self.channel)
    }

    pub(crate) fn get_message(&self) -> MessageId {
        MessageId(self.message)
    }

    pub(crate) fn set_message(&mut self, id: u64) {
        self.message = id;
    }

    pub(crate) fn get_link(&self) -> String {
        self.link.clone()
    }

    pub(crate) fn set_link(&mut self, link: String) {
        self.link = link;
    }

    pub(crate) fn get_update(&self) -> u64 {
        self.updated
    }

    pub(crate) fn set_update(&mut self, update: u64) {
        self.updated = update;
    }

    pub(crate) fn cache(&mut self, id: u64, message: Message) {
        if let Some(link) = message.link {
            self.cache.put(link, (id, *md5::compute(message.content)));
        }
    }

    pub(crate) fn check_changed(&self, message: &Message) -> Option<u64> {
        if let Some(link) = &message.link {
            if let Some(old_msg) = self.cache.get(link) {
                let (old_id, old_hash) = old_msg;

                if &*md5::compute(&message.content) != old_hash {
                    Some(*old_id)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
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

        // Create `Message`s from fetched RSS messages
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

                    Some(Message::new(
                        match &m.author {
                            Some(a) => match reg_author.captures(a) {
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
                                .unwrap_or_else(|| "Trigobot for President".to_owned()),
                        ),
                        html2md::parse_html(
                            &m.description
                                .clone()
                                .unwrap_or_else(|| "I am inevitable".to_owned()),
                        ),
                        m.link.clone(),
                        ts,
                    ))
                } else {
                    None
                }
            })
            .collect();

        result.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        self.updated = if let Some(last) = result.last() {
            last.timestamp
        } else {
            self.updated
        };

        result
    }
}

type MessageInfo = (u64, [u8; 16]);
