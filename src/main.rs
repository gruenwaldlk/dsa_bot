mod commands;
mod dsa;
mod lib;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate json_gettext;

use json_gettext::JSONGetText;
use log::debug;
use log::error;
use log::info;
use log::warn;
use regex::Regex;
use sentry::init;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use commands::character::*;
use commands::meta::*;
use commands::roll::*;
use dsa::char_repository::CharRepository;

lazy_static! {
    static ref BASIC_DICE_REGEX: Regex =
        Regex::new(r"^(\d+)(d|w)(\d+)$").expect("The regex could not be parsed.");
    static ref BASIC_DICE_REGEX_WITH_MOD: Regex =
        Regex::new(r"^(\d+)(w|d)(\d+)(\+|-)(\d+)$").expect("The regex could not be parsed.");
    static ref BASIC_TALENT_ROLL_REGEX: Regex =
        Regex::new(r"^([a-zA-Zöäü_]+)$").expect("The regex could not be parsed.");
    static ref BASIC_TALENT_ROLL_REGEX_WITH_MOD: Regex =
        Regex::new(r"^([a-zA-Zöäüß_]+)(\+|-)(\d+)$").expect("The regex could not be parsed.");
    static ref CHARACTER_REPOSITORY: CharRepository = CharRepository::new_from_file(
        &env::var("CHARACTER_REPOSITORY").expect("The environment variable could not be found.")

    );
    static ref CTX: JSONGetText<'static> = static_json_gettext_build!(
        "en_GB",
        "en_GB", "lang/en_GB.json",
        "de_DE", "lang/de_DE.json"
    ).unwrap();
    /*
    static ref TEXTS: Catalog = Catalog::parse(
        File::open(&format!(
            "{}.mo",
            env::var("LANG").expect("The environment variable could not be found.")
        ))
        .expect("The file could not be found.")
    )
    .expect("The texts file could not be parsed.");
    */
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct EvtHandler;

impl EventHandler for EvtHandler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(
    ping,
    roll,
    roll_sum_mod,
    character_info,
    talent_roll,
    get_ini,
    help,
    uwu
)]
struct General;

fn main() {
    kankyo::load(false).expect("Failed to load .env file");
    let sentry_token = env::var("SENTRY_TOKEN").unwrap_or_else(|_| String::from(""));
    let sentry = sentry::init((
        sentry_token.as_str(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));
    if sentry.is_enabled() {
        sentry::integrations::env_logger::init(None, Default::default());
        sentry::integrations::panic::register_panic_handler();
        info!("Sentry integration initialised.");
    } else {
        env_logger::init();
        warn!("Sentry could not be initialised - please provide a valid sentry token in .env:SENTRY_TOKEN");
    }
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, EvtHandler).expect("Error creating the Discord client.");
    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }
    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };
    let prefix = env::var("COMMAND_PREFIX").expect("Expected a prefix in the environment");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix(&prefix))
            .group(&GENERAL_GROUP),
    );
    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
pub(crate) fn curr_lang() -> String {
    let current_language =
        env::var("CURRENT_LANGUAGE").expect("Expected a token in the environment");
    debug!("Current language: {}", current_language);
    current_language
}

mod test {
    use super::CTX;
    use super::*;
    #[test]
    fn test_loc() {
        assert_eq!("Test: en_GB", get_text!(CTX, "test").unwrap());
        assert_eq!("Test: en_GB", get_text!(CTX, "en_GB", "test").unwrap());
        assert_eq!("Test: de_DE", get_text!(CTX, "de_DE", "test").unwrap());
    }

    #[test]
    fn test_loc_with_curr_lang() {
        kankyo::load(false).expect("Failed to load .env file");
        assert_eq!("Test: de_DE", get_text!(CTX, curr_lang(), "test").unwrap());
    }
}
