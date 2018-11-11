use clap::Arg;

use crate::args::{
    hexagram, trigram,
};

pub const HELP: &str = "(A 6 digit number){n}Cast your own coins and enter the sequence as a series of digits.{n} ";
pub const LONG_HELP: &str = "\
For each line, cast 3 coins. Heads count for 3 and tails count for 2. Add up the \
result of casting 3 coins to get a number between 6 and 9. After casting enough \
coins for 6 lines, enter the results as a series of digits.{n}
example:{n}iching -c 667987{n} ";
pub const LONG: &str = NAME;
pub const NAME: &str = "cast";
pub const SHORT: &str = "c";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "COIN TOSS RESULTS";

pub fn declare_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(NAME)
        .conflicts_with_all(&[trigram::NAME, hexagram::NAME])
        .short(SHORT)
        .long(LONG)
        .value_name(VALUE_NAME)
        .help(HELP)
        .long_help(LONG_HELP)
        .takes_value(TAKES_VALUE)
}