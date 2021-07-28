use std::env::var as ENV;

use dotenv::dotenv;

/// Environment testing and error handling

/// Variables that can be used
pub enum Variables {
    AnnouncementIcon,
    AnnouncementsChannel,
    CommandPrefix,
    DelegateRole,
    DiscordToken,
    DomainsFile,
    PinMinReactions,
    PinReaction,
    ReactionRole,
    ReactionRolesChannel,
    RulesChannel,
    StateFile,
    WelcomeChannel,
}

const VAR_ANNOUNCE_CHANNEL: &str = "ANNOUNCEMENTS";
const VAR_ANNOUNCE_ICON: &str = "ANNOUNCEMENT_ICON";
const VAR_COMMAND_PREFIX: &str = "COMMAND_PREFIX";
const VAR_DELEGATE_ROLE: &str = "DELEGATE_ROLE";
const VAR_DISCORD_TOKEN: &str = "DISCORD_TOKEN";
const VAR_DOMAINS_FILE: &str = "PEERS_FILE";
const VAR_PIN_REACTION: &str = "PIN_REACTION";
const VAR_PIN_MIN_REACTIONS: &str = "PIN_MIN_REACTIONS";
const VAR_REACT_ROLE: &str = "REACT_ROLE";
const VAR_REACT_ROLE_CHANNEL: &str = "REACT_ROLE_CHANNEL";
const VAR_RULES_CHANNEL: &str = "RULES_CHANNEL";
const VAR_STATE_FILE: &str = "STATE_FILE";
const VAR_WELCOME_CHANNEL: &str = "WELCOME_CHANNEL";

pub fn test_env() {
    // Load .env file vars
    dotenv().ok();

    // Try all enum variations
    get_var(Variables::AnnouncementIcon);
    get_var(Variables::AnnouncementsChannel);
    get_var(Variables::CommandPrefix);
    get_var(Variables::DelegateRole);
    get_var(Variables::DiscordToken);
    get_var(Variables::DomainsFile);
    get_var(Variables::PinMinReactions);
    get_var(Variables::PinReaction);
    get_var(Variables::ReactionRole);
    get_var(Variables::ReactionRolesChannel);
    get_var(Variables::RulesChannel);
    get_var(Variables::StateFile);
    get_var(Variables::WelcomeChannel);
}

pub fn get_var(var: Variables) -> String {
    match ENV(match var {
        Variables::AnnouncementIcon => VAR_ANNOUNCE_ICON,
        Variables::AnnouncementsChannel => VAR_ANNOUNCE_CHANNEL,
        Variables::CommandPrefix => VAR_COMMAND_PREFIX,
        Variables::DelegateRole => VAR_DELEGATE_ROLE,
        Variables::DiscordToken => VAR_DISCORD_TOKEN,
        Variables::DomainsFile => VAR_DOMAINS_FILE,
        Variables::PinMinReactions => VAR_PIN_MIN_REACTIONS,
        Variables::PinReaction => VAR_PIN_REACTION,
        Variables::ReactionRole => VAR_REACT_ROLE,
        Variables::ReactionRolesChannel => VAR_REACT_ROLE_CHANNEL,
        Variables::RulesChannel => VAR_RULES_CHANNEL,
        Variables::StateFile => VAR_STATE_FILE,
        Variables::WelcomeChannel => VAR_WELCOME_CHANNEL,
    }) {
        Ok(val) => val,
        Err(_) => panic!("{}", get_error(var)),
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
        Variables::RulesChannel => format!(
            "No rules channel defined!\nSet env var {} with the id of the channel",
            VAR_RULES_CHANNEL
        ),
        Variables::ReactionRolesChannel => format!(
            "No reaction roles channel defined!\nSet env var {} with the id of the channel",
            VAR_REACT_ROLE_CHANNEL
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
