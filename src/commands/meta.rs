use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.reply(ctx, "Pong!");
    Ok(())
}

#[command]
fn uwu(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.reply(ctx, "Fuck off!");
    Ok(())
}

#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.reply(ctx, get_help_text());
    Ok(())
}

fn get_help_text() -> String {
    let mut string = String::new();
    string.push_str("**Commands**\n```txt\n");
    string.push_str("roll         <n>d<m>          Rolls n m-sided dice.\n");
    string.push_str("                              Shorthand: r\n");
    string.push_str("roll_sum_mod <x>d<y>+|-<z>    Rolls x y-sided dice with an optional modificator z and sums them up.\n");
    string.push_str(
        "                              Shorthand: rsm, rsmod, rsummod, roll_sum, rsum, rs\n",
    );
    string.push_str("character_info                Rolls x y-sided dice with an optional modificator z and sums them up.\n");
    string.push_str("                              Shorthand: char, c\n");
    string.push_str(
        "talent_roll <talent>+|-<m>    Performs a talent check with an optional modification m.\n",
    );
    string.push_str("                              Shorthand: t\n");
    string.push_str("get_ini                       Rolls the character's ini.\n");
    string.push_str("                              Shorthand: ini\n");
    string.push_str("```");
    string
}
