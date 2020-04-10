use crate::lib::add_without_overflow;
use crate::lib::dice::Dice;
use crate::lib::substract_without_overflow;
use crate::lib::Operator;
use log::error;
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
    if let Err(why) = msg.reply(&ctx.http, result) {
        error!("Error sending message: {:?}", why);
    }
    Ok(())
}

fn roll_dice(args: &str) -> String {
    if !crate::BASIC_DICE_REGEX.is_match(args) || args.len() < 3 {
        error!("Invalid dice pattern {:?}", args);
        return format!("Invalid dice pattern {:?}", args);
    }
    let mut dice_type = 6;
    let mut dice_count = 1;
    for cap in crate::BASIC_DICE_REGEX.captures_iter(args) {
        dice_count = u8::from_str(&cap[1]).unwrap_or(1);
        dice_type = u8::from_str(&cap[3]).unwrap_or(6);
    }
    if dice_count < 1 {
        return format!("Are you sure you want me to roll {} dice?", dice_count);
    }
    if dice_type < 2 {
        return format!(
            "Are you sure you want me to roll a dice with {} side(-s)?",
            dice_type
        );
    }
    let dice = Dice::new(dice_type);
    if dice_count > 1 {
        let mut vec: Vec<u8> = Vec::new();
        dice.roll_n_times(dice_count, &mut vec);
        format!("I rolled {} {} for you: {:?}", dice_count, dice, vec)
    } else {
        format!("I rolled {} {} for you: {}", dice_count, dice, dice.roll())
    }
}

#[command]
#[aliases("rsm", "rsmod", "rsummod", "roll_sum", "rs", "rsum")]
fn roll_sum_mod(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let result = roll_dice_sum(args.current().unwrap_or(""));
    if let Err(why) = msg.reply(&ctx.http, result) {
        error!("Error sending message: {:?}", why);
    }
    Ok(())
}

fn roll_dice_sum(args: &str) -> String {
    if !(crate::BASIC_DICE_REGEX.is_match(args) || crate::BASIC_DICE_REGEX_WITH_MOD.is_match(args))
    {
        error!("Invalid dice pattern {:?}", args);
        return format!("Invalid dice pattern {:?}", args);
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
        return format!("Are you sure you want me to roll {} dice?", dice_count);
    }
    if dice_type < 2 {
        return format!(
            "Are you sure you want me to roll a dice with {} side(-s)?",
            dice_type
        );
    }
    let mut ret_str = String::new();
    ret_str.push_str("Result: ");
    let mut result = 0;
    let mut v: Vec<u8> = Vec::new();
    Dice::new(dice_type).roll_n_times(dice_count, &mut v);
    for x in v.iter() {
        result = add_without_overflow(result, *x);
    }
    ret_str.push_str(&u8::to_string(&result));
    ret_str
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
        return format!("Are you sure you want me to roll {} dice?", dice_count);
    }
    if dice_type < 2 {
        return format!(
            "Are you sure you want me to roll a dice with {} side(-s)?",
            dice_type
        );
    }
    let mut ret_str = String::new();
    ret_str.push_str("Result: ");
    let mut result = 0;
    let mut v: Vec<u8> = Vec::new();
    Dice::new(dice_type).roll_n_times(dice_count, &mut v);
    for x in v.iter() {
        result = add_without_overflow(result, *x);
    }
    ret_str.push_str(&u8::to_string(&result));
    if op == Operator::Plus {
        result = add_without_overflow(result, modifier);
    } else {
        result = substract_without_overflow(result, modifier);
    }
    ret_str.push_str(&u8::to_string(&result));
    ret_str
}
