use crate::util::localisation::get_text;
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
#[aliases("h", "?")]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.reply(ctx, get_help_text());
    Ok(())
}

fn get_help_text() -> String {
    get_text("commands.meta.help")
}
