# DSA Bot - Phex's Bote - Phex's Messenger

Phex's messenger is a bot built to support playing the pen & paper The Dark Eye/Das schwarze Auge.
It currently supports basic (arbitrary) dice rolls and talent rolls for player characters.

## Getting Started

### The `.ENV` File

In order to run the bot it requires a file called `.env` placed next to the executable with the following contents:

```ini
# The discord app token. Do **not ever** publish this token.
DISCORD_TOKEN=your-token-here
# The prefix for bot commands.
COMMAND_PREFIX=!
# The relative path to the character repository.
CHARACTER_REPOSITORY=.chars
# Declares the level of logging to use. Read the documentation for the `log`
# and `env_logger` crates for more information.
RUST_LOG=info
```

### Characters

Characters exported as JSON file can be dumped into the directory specified in the `CHARACTER_REPOSITORY` and will be
loaded automatically.

## Features

### Commands

|Command|Shorthand|Argument|Argument Regex|Description|
|:--|:--|:--|:--|:--|
|`roll`|`r`|`<n>d<m>`|`^(\d+)(d|w)(\d+)$`|Rolls n m-sided dice: e.g. `!r 3d20` or `!roll 2d6`.|
|`roll_sum_mod`| `rsm, rsmod, rsummod, roll_sum, rsum, rs` |`<x>d<y>+|-<z>`| `^(\d+)(d|w)(\d+)(\+|-)(\d+)$` | Rolls `x` `y`-sided dice with an optional mod `z` and sums them up, e.g. `!rsm 3d9+5` |
|`character_info`|`c`| | |Tells a player as what character they are playing.|
|`talent_roll` | `t` | `<talent>+/-<m>`| `^([a-zA-Zöäüß_]+)(\+|-)(\d+)$` | Performs a talent check with an optional modification, eg. `!betoeren` or `!fliegen+3`
|`get_ini` | `ini` | | |Rolls the character's initiative.|

## Shortcomings

* The bot currently only supports talent rolls in German.
* The bot is self-hosted only.
