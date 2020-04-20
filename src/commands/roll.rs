use crate::curr_lang;
use crate::lib::add_without_overflow;
use crate::lib::dice::Dice;
use crate::lib::subtract_without_overflow;
use crate::lib::Operator;
use crate::CTX;
use log::debug;
use log::error;
use log::info;
use log::warn;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::str::FromStr;

#[command]
#[aliases("roll", "r")]
fn roll(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let result = roll_dice(args.current().unwrap_or(""));
    debug!("Generated Discord message (@{}): {}", msg.author, result);
    if let Err(why) = msg.reply(&ctx.http, result) {
        error!("Error sending message: {:?}", why);
    }
    Ok(())
}

fn roll_dice(args: &str) -> String {
    if !crate::BASIC_DICE_REGEX.is_match(args) || args.len() < 3 {
        warn!("Invalid dice pattern {:?}", args);
        return err_invalid_dice_pattern(args);
    }
    let mut dice_type = 6;
    let mut dice_count = 1;
    for cap in crate::BASIC_DICE_REGEX.captures_iter(args) {
        dice_count = u8::from_str(&cap[1]).unwrap_or(1);
        dice_type = u8::from_str(&cap[3]).unwrap_or(6);
    }
    if dice_count < 1 {
        return err_invalid_dice_count(dice_count);
    }
    if dice_type < 2 {
        return err_invalid_dice(dice_type);
    }
    let dice = Dice::new(dice_type);
    if dice_count > 1 {
        let mut vec: Vec<u8> = Vec::new();
        dice.roll_n_times(dice_count, &mut vec);
        format!(
            "{}{} {}{}{:?}",
            get_text!(CTX, curr_lang(), "commands.roll.i-rolled").unwrap(),
            dice_count,
            dice,
            get_text!(CTX, curr_lang(), "commands.roll.for-you").unwrap(),
            vec
        )
    } else {
        format!(
            "{}{}{}{}",
            get_text!(CTX, curr_lang(), "commands.roll.i-rolled-a").unwrap(),
            dice,
            get_text!(CTX, curr_lang(), "commands.roll.for-you").unwrap(),
            dice.roll()
        )
    }
}

#[command]
#[aliases("rsm", "rsmod", "rsummod", "roll_sum", "rs", "rsum")]
fn roll_sum_mod(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let result = roll_dice_sum(args.current().unwrap_or(""));
    debug!("Generated Discord message (@{}): {}", msg.author, result);
    if let Err(why) = msg.reply(&ctx.http, result) {
        error!("Error sending message: {:?}", why);
    }
    Ok(())
}

fn roll_dice_sum(args: &str) -> String {
    if !(crate::BASIC_DICE_REGEX.is_match(args) || crate::BASIC_DICE_REGEX_WITH_MOD.is_match(args))
    {
        warn!("Invalid dice pattern {:?}", args);
        return err_invalid_dice_pattern(args);
    }

    if crate::BASIC_DICE_REGEX_WITH_MOD.is_match(args) {
        roll_dice_sum_mod(args)
    } else {
        roll_dice_sum_no_mod(args)
    }
}

fn roll_dice_sum_no_mod(args: &str) -> String {
    let mut dice_type = 6;
    let mut dice_count = 1;

    for cap in crate::BASIC_DICE_REGEX.captures_iter(args) {
        dice_count = u8::from_str(&cap[1]).unwrap_or(1);
        dice_type = u8::from_str(&cap[3]).unwrap_or(6);
    }
    if dice_count < 1 {
        return err_invalid_dice_count(dice_count);
    }
    if dice_type < 2 {
        return err_invalid_dice(dice_type);
    }
    let mut result = 0;
    let mut v: Vec<u8> = Vec::new();
    Dice::new(dice_type).roll_n_times(dice_count, &mut v);
    for x in v.iter() {
        result = add_without_overflow(result, *x);
    }
    if dice_count > 1 {
        format!(
            "{}{} {}{}{:?}",
            get_text!(CTX, curr_lang(), "commands.roll.i-rolled").unwrap(),
            dice_count,
            Dice::new(dice_type),
            get_text!(CTX, curr_lang(), "commands.roll.for-you").unwrap(),
            result
        )
    } else {
        format!(
            "{}{}{}{}",
            get_text!(CTX, curr_lang(), "commands.roll.i-rolled-a").unwrap(),
            Dice::new(dice_type),
            get_text!(CTX, curr_lang(), "commands.roll.for-you").unwrap(),
            result
        )
    }
}

fn roll_dice_sum_mod(args: &str) -> String {
    let mut modifier = 0;
    let mut dice_type = 6;
    let mut dice_count = 1;
    let mut op = Operator::Plus;

    for cap in crate::BASIC_DICE_REGEX_WITH_MOD.captures_iter(args) {
        dice_count = u8::from_str(&cap[1]).unwrap_or(1);
        dice_type = u8::from_str(&cap[3]).unwrap_or(6);
        if &cap[4] == "+" {
            op = Operator::Plus;
        } else {
            op = Operator::Minus;
        }
        modifier = u8::from_str(&cap[5]).unwrap_or(0);
    }
    if dice_count < 1 {
        return err_invalid_dice_count(dice_count);
    }
    if dice_type < 2 {
        return err_invalid_dice(dice_type);
    }
    info!("Rolling {}w{}{:?}{}.", dice_count, dice_type, op, modifier);
    let mut result = 0;
    let mut v: Vec<u8> = Vec::new();
    Dice::new(dice_type).roll_n_times(dice_count, &mut v);
    for x in v.iter() {
        result = add_without_overflow(result, *x);
    }
    if op == Operator::Plus {
        result = add_without_overflow(result, modifier);
    } else {
        result = subtract_without_overflow(result, modifier);
    }
    if dice_count > 1 {
        format!(
            "{}{}{}{}{}{}",
            get_text!(CTX, curr_lang(), "commands.roll.i-rolled").unwrap(),
            dice_count,
            Dice::new(dice_type),
            with_mod(op, modifier),
            get_text!(CTX, curr_lang(), "commands.roll.for-you").unwrap(),
            result
        )
    } else {
        format!(
            "{}{}{}{}{}",
            get_text!(CTX, curr_lang(), "commands.roll.i-rolled-a").unwrap(),
            Dice::new(dice_type),
            with_mod(op, modifier),
            get_text!(CTX, curr_lang(), "commands.roll.for-you").unwrap(),
            result
        )
    }
}

fn with_mod(op: Operator, modifier: u8) -> String {
    let operator = if op == Operator::Plus { "+" } else { "-" };
    format!("{}{}", operator, modifier)
}

fn err_invalid_dice_pattern(args: &str) -> String {
    format!(
        "{} {:?}",
        get_text!(CTX, curr_lang(), "commands.roll.err.invalid-dice-pattern").unwrap(),
        args
    )
}

fn err_invalid_dice_count(dice_count: u8) -> String {
    format!(
        "{}{}",
        get_text!(CTX, curr_lang(), "commands.roll.err.invalid-dice-count").unwrap(),
        dice_count
    )
}

fn err_invalid_dice(dice_type: u8) -> String {
    format!(
        "{}d{}",
        get_text!(CTX, curr_lang(), "commands.roll.err.invalid-dice").unwrap(),
        dice_type
    )
}

mod test {
    use super::*;
    #[test]
    fn test_roll_dice_sum_mod() {
        let s = roll_dice_sum_mod("1w6");
        println!("Rolled {}", s);
        let s = roll_dice_sum_mod("2w6");
        println!("Rolled {}", s);
        let s = roll_dice_sum_mod("3w6");
        println!("Rolled {}", s);
        let s = roll_dice_sum_mod("1w6+1");
        println!("Rolled {}", s);
        let s = roll_dice_sum_mod("2w6+2");
        println!("Rolled {}", s);
        let s = roll_dice_sum_mod("3w6+3");
        println!("Rolled {}", s);
    }
}
