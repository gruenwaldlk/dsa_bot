mod commands;
mod dsa;
mod lib;

#[macro_use]
extern crate lazy_static;

use log::error;
use log::info;
use regex::Regex;
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
        Regex::new(r"^([a-zA-Zöäü_]+)(\+|-)(\d+)$").expect("The regex could not be parsed.");
    static ref CHARACTER_REPOSITORY: CharRepository = CharRepository::new_from_file(
        &env::var("CHARACTER_REPOSITORY").expect("The environment variable could not be found.")
    );
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
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, EvtHandler).expect("Err creating client");

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
