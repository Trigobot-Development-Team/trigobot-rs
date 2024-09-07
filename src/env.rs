use std::collections::HashMap;
use std::env;
use std::sync::mpsc::channel;
use std::sync::RwLock;
use std::time::Duration;

use dotenv::dotenv;

use lazy_static::lazy_static;

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

use serenity::Error;

use tokio::task;

/// Environment testing and error handling

/// Variables that can be used
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Variables {
    AnnouncementIcon,
    AnnouncementsChannel,
    CacheEntries,
    CommandPrefix,
    DelegateRole,
    DiscordToken,
    DomainsFile,
    PinMinReactions,
    PinReaction,
    ReactionRole,
    ReactionRolesChannel,
    RssSleep,
    RulesChannel,
    StateFile,
    WelcomeChannel,
}

const VAR_ANNOUNCE_CHANNEL: &str = "ANNOUNCEMENTS";
const VAR_ANNOUNCE_ICON: &str = "ANNOUNCEMENT_ICON";
const VAR_CACHE_ENTRIES: &str = "CACHE_ENTRIES";
const VAR_COMMAND_PREFIX: &str = "COMMAND_PREFIX";
const VAR_DELEGATE_ROLE: &str = "DELEGATE_ROLE";
const VAR_DISCORD_TOKEN: &str = "DISCORD_TOKEN";
const VAR_DOMAINS_FILE: &str = "PEERS_FILE";
const VAR_PIN_REACTION: &str = "PIN_REACTION";
const VAR_PIN_MIN_REACTIONS: &str = "PIN_MIN_REACTIONS";
const VAR_REACT_ROLE: &str = "REACT_ROLE";
const VAR_REACT_ROLE_CHANNEL: &str = "REACT_ROLE_CHANNEL";
const VAR_RSS_SLEEP: &str = "RSS_SLEEP";
const VAR_RULES_CHANNEL: &str = "RULES_CHANNEL";
const VAR_STATE_FILE: &str = "STATE_FILE";
const VAR_WELCOME_CHANNEL: &str = "WELCOME_CHANNEL";

lazy_static! {
    static ref BOT_CONFIG: RwLock<HashMap<Variables, String>> = RwLock::new(HashMap::new());
}

/// Live reload config
// Return type just to make the compiler happy
pub async fn check_env() -> Result<(), Error> {
    task::spawn_blocking(move || {
        let (tx, rx) = channel();

        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

        watcher.watch(".env", RecursiveMode::NonRecursive).unwrap();

        loop {
            match rx.recv() {
                Ok(event) => {
                    if let DebouncedEvent::Write(_) = event {
                        clean_env();
                        load_env();
                    }
                }
                Err(e) => eprintln!("Watch error: {}", e),
            }
        }
    });

    Ok(())
}

fn clean_env() {
    env::remove_var(get_var_name(Variables::AnnouncementIcon));
    env::remove_var(get_var_name(Variables::AnnouncementsChannel));
    env::remove_var(get_var_name(Variables::CacheEntries));
    env::remove_var(get_var_name(Variables::CommandPrefix));
    env::remove_var(get_var_name(Variables::DelegateRole));
    env::remove_var(get_var_name(Variables::DiscordToken));
    env::remove_var(get_var_name(Variables::DomainsFile));
    env::remove_var(get_var_name(Variables::PinMinReactions));
    env::remove_var(get_var_name(Variables::PinReaction));
    env::remove_var(get_var_name(Variables::ReactionRole));
    env::remove_var(get_var_name(Variables::ReactionRolesChannel));
    env::remove_var(get_var_name(Variables::RssSleep));
    env::remove_var(get_var_name(Variables::RulesChannel));
    env::remove_var(get_var_name(Variables::StateFile));
    env::remove_var(get_var_name(Variables::WelcomeChannel));
}

pub fn load_env() {
    // Load .env file vars
    dotenv().expect("failed to load variables from .env");

    // Try all enum variations
    let mut new_vals = HashMap::new();
    new_vals.insert(
        Variables::AnnouncementIcon,
        get_var_from_env(Variables::AnnouncementIcon),
    );
    new_vals.insert(
        Variables::AnnouncementsChannel,
        get_var_from_env(Variables::AnnouncementsChannel),
    );
    new_vals.insert(
        Variables::CacheEntries,
        get_var_from_env(Variables::CacheEntries),
    );
    new_vals.insert(
        Variables::CommandPrefix,
        get_var_from_env(Variables::CommandPrefix),
    );
    new_vals.insert(
        Variables::DelegateRole,
        get_var_from_env(Variables::DelegateRole),
    );
    new_vals.insert(
        Variables::DiscordToken,
        get_var_from_env(Variables::DiscordToken),
    );
    new_vals.insert(
        Variables::DomainsFile,
        get_var_from_env(Variables::DomainsFile),
    );
    new_vals.insert(
        Variables::PinMinReactions,
        get_var_from_env(Variables::PinMinReactions),
    );
    new_vals.insert(
        Variables::PinReaction,
        get_var_from_env(Variables::PinReaction),
    );
    new_vals.insert(
        Variables::ReactionRole,
        get_var_from_env(Variables::ReactionRole),
    );
    new_vals.insert(
        Variables::ReactionRolesChannel,
        get_var_from_env(Variables::ReactionRolesChannel),
    );
    new_vals.insert(Variables::RssSleep, get_var_from_env(Variables::RssSleep));
    new_vals.insert(
        Variables::RulesChannel,
        get_var_from_env(Variables::RulesChannel),
    );
    new_vals.insert(Variables::StateFile, get_var_from_env(Variables::StateFile));
    new_vals.insert(
        Variables::WelcomeChannel,
        get_var_from_env(Variables::WelcomeChannel),
    );

    let mut lock = match BOT_CONFIG.write() {
        Ok(val) => val,
        Err(e) => panic!("Poisoned config lock: {}", e),
    };

    lock.clear();

    lock.extend(new_vals);
}

pub fn get_var(var: Variables) -> String {
    let lock = match BOT_CONFIG.read() {
        Ok(val) => val,
        Err(e) => panic!("Poisoned config lock: {}", e),
    };

    match lock.get(&var) {
        None => panic!("{}", get_error(var)),
        Some(val) => val.to_owned(),
    }
}

fn get_var_from_env(var: Variables) -> String {
    match env::var(get_var_name(var)) {
        Ok(val) => val,
        Err(_) => panic!("{}", get_error(var)),
    }
}

fn get_var_name(var: Variables) -> &'static str {
    match var {
        Variables::AnnouncementIcon => VAR_ANNOUNCE_ICON,
        Variables::AnnouncementsChannel => VAR_ANNOUNCE_CHANNEL,
        Variables::CacheEntries => VAR_CACHE_ENTRIES,
        Variables::CommandPrefix => VAR_COMMAND_PREFIX,
        Variables::DelegateRole => VAR_DELEGATE_ROLE,
        Variables::DiscordToken => VAR_DISCORD_TOKEN,
        Variables::DomainsFile => VAR_DOMAINS_FILE,
        Variables::PinMinReactions => VAR_PIN_MIN_REACTIONS,
        Variables::PinReaction => VAR_PIN_REACTION,
        Variables::ReactionRole => VAR_REACT_ROLE,
        Variables::ReactionRolesChannel => VAR_REACT_ROLE_CHANNEL,
        Variables::RssSleep => VAR_RSS_SLEEP,
        Variables::RulesChannel => VAR_RULES_CHANNEL,
        Variables::StateFile => VAR_STATE_FILE,
        Variables::WelcomeChannel => VAR_WELCOME_CHANNEL,
    }
}

fn get_error(var: Variables) -> String {
    match var {
        Variables::AnnouncementIcon => format!(
            "No announcement icon defined!\nSet env var {} with a link to the image",
            VAR_ANNOUNCE_ICON
        ),
        Variables::AnnouncementsChannel => format!(
            "No announcements channel defined!\nSet env var {} with the Discord channel id",
            VAR_ANNOUNCE_CHANNEL
        ),
        Variables::CacheEntries => format!(
            "No number of cache entries defined!\nSet env var {} with an appropriate value",
            VAR_CACHE_ENTRIES
        ),
        Variables::CommandPrefix => format!(
            "No command prefix defined!\nSet env var {} with the prefix",
            VAR_COMMAND_PREFIX
        ),
        Variables::DelegateRole => format!(
            "No delegate role defined|\nSet env var {}, with the id of the role",
            VAR_DELEGATE_ROLE
        ),
        Variables::DiscordToken => format!(
            "No Discord token found!\nSet env var {} with the token",
            VAR_DISCORD_TOKEN
        ),
        Variables::DomainsFile => format!(
            "Unknown domains file!\nSet env var {} with the path to the appropriate file",
            VAR_DOMAINS_FILE
        ),
        Variables::PinMinReactions => format!(
            "Unknown minimum number of reactions to pin message!\nSet env var {} with the chosen value",
            VAR_PIN_MIN_REACTIONS
        ),
        Variables::PinReaction => format!(
            "No pin reaction defined!\nSet env var {} with the chosen reaction",
            VAR_PIN_REACTION
        ),
        Variables::ReactionRole => format!(
            "No reaction defined for roles!\nSet env var {} with the chosen reaction",
            VAR_REACT_ROLE
        ),
        Variables::ReactionRolesChannel => format!(
            "No reaction roles channel defined!\nSet env var {} with the id of the channel",
            VAR_REACT_ROLE_CHANNEL
        ),
        Variables::RssSleep => format!(
            "No RSS sleep time defined!\nSet env var {} with the appropriate value",
            VAR_RSS_SLEEP
        ),
        Variables::RulesChannel => format!(
            "No rules channel defined!\nSet env var {} with the id of the channel",
            VAR_RULES_CHANNEL
        ),
        Variables::StateFile => format!(
            "Unknown state file!\nSet env var {} with the path to the appropriate file",
            VAR_STATE_FILE
        ),
        Variables::WelcomeChannel => format!(
            "No welcome channel defined!\nSet env var {} with the id of the channel",
            VAR_WELCOME_CHANNEL
        ),
    }
}
