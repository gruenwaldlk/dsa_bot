use crate::commands::util;
use crate::lib;
use crate::util::localisation::get_text;
use log::error;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::str::FromStr;

#[command]
#[aliases("char", "c")]
fn character_info(ctx: &mut Context, msg: &Message) -> CommandResult {
    let is_dm = match &msg.member {
        Some(pm) => util::is_dm(pm),
        _ => false,
    };
    let result = match crate::CHARACTER_REPOSITORY.get_char_by_player_id(*msg.author.id.as_u64()) {
        Some(c) => {
            if is_dm {
                get_text("commands.character.you-are-dm")
            } else {
                format!(
                    "{}{}.",
                    get_text("commands.character.you-are-playing-as"),
                    c.name()
                )
            }
        }
        None => {
            if is_dm {
                get_text("commands.character.you-are-dm")
            } else {
                get_text("commands.character.you-are-not-playing-as-anyone")
            }
        }
    };
    if let Err(why) = msg.reply(&ctx.http, result) {
        error!("Error sending message: {:?}", why);
    }
    Ok(())
}

#[command]
#[aliases("t")]
fn talent_roll(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let is_dm = match &msg.member {
        Some(pm) => util::is_dm(pm),
        _ => false,
    };
    if is_dm {
        if let Err(why) = msg.reply(
            &ctx.http,
            format!(
                "{} {}",
                get_text("commands.character.you-are-dm"),
                get_text("commands.character.no-talent-checks-allowed")
            ),
        ) {
            error!("Error sending message: {:?}", why);
        }
        return Ok(());
    }
    let c = match crate::CHARACTER_REPOSITORY.get_char_by_player_id(*msg.author.id.as_u64()) {
        Some(c) => c,
        None => {
            if let Err(why) = msg.reply(
                &ctx.http,
                get_text("commands.character.you-are-not-playing-as-anyone"),
            ) {
                error!("Error sending message: {:?}", why);
            }
            return Ok(());
        }
    };
    match TalentCheck::from_str(args.message()) {
        Some(check) => {
            if let Err(why) = msg.reply(
                &ctx.http,
                c.check_talent(check.talent_id(), check.mod_value(), check.mod_op()),
            ) {
                error!("Error sending message: {:?}", why);
            }
            Ok(())
        }
        None => {
            if let Err(why) = msg.reply(
                &ctx.http,
                format!(
                    "{} {}",
                    get_text("commands.character.could-not-parse-talent-check"),
                    args.message()
                ),
            ) {
                error!("Error sending message: {:?}", why);
            }
            Ok(())
        }
    }
}

#[command]
#[aliases("ini")]
fn get_ini(ctx: &mut Context, msg: &Message) -> CommandResult {
    let c = match crate::CHARACTER_REPOSITORY.get_char_by_player_id(*msg.author.id.as_u64()) {
        Some(c) => c,
        None => {
            if let Err(why) = msg.reply(
                &ctx.http,
                get_text("commands.character.you-are-not-playing-as-anyone"),
            ) {
                error!("Error sending message: {:?}", why);
            }
            return Ok(());
        }
    };
    if let Err(why) = msg.reply(
        &ctx.http,
        format!("{}{}", get_text("commands.character.ini-player"), c.ini()),
    ) {
        error!("Error sending message: {:?}", why);
    }
    Ok(())
}

struct TalentCheck {
    pub(self) talent_id: String,
    pub(self) mod_value: u8,
    pub(self) mod_op: lib::operator::Operator,
}

impl TalentCheck {
    fn talent_id(&self) -> &str {
        &self.talent_id
    }
    fn mod_value(&self) -> u8 {
        self.mod_value
    }
    fn mod_op(&self) -> lib::operator::Operator {
        self.mod_op
    }
    fn from_str(s: &str) -> Option<Self> {
        if crate::BASIC_TALENT_ROLL_REGEX.is_match(s) {
            Some(TalentCheck {
                talent_id: String::from(s),
                mod_value: 0,
                mod_op: lib::operator::Operator::NoP,
            })
        } else if crate::BASIC_TALENT_ROLL_REGEX_WITH_MOD.is_match(s) {
            let mut id = String::from("NONE");
            let mut op = lib::operator::Operator::NoP;
            let mut val = 0;
            for cap in crate::BASIC_TALENT_ROLL_REGEX_WITH_MOD.captures_iter(s) {
                id = String::from(&cap[1]);
                val = u8::from_str(&cap[3]).unwrap_or(0);
                if &cap[2] == "+" {
                    op = lib::operator::Operator::Plus;
                } else if &cap[2] == "-" {
                    op = lib::operator::Operator::Minus;
                } else {
                    op = lib::operator::Operator::NoP;
                }
            }
            Some(TalentCheck {
                talent_id: id,
                mod_value: val,
                mod_op: op,
            })
        } else {
            None
        }
    }
}
