use clap::Arg;

use crate::args::{
    cast, hexagram, question,
};

pub const HELP: &str = "(A number 1-8){n}Look up a trigram by number.{n} ";
pub const LONG_HELP: &str = "\
Look up a trigram by number.{n}
example:{n}iching -t 6{n} ";
pub const LONG: &str = NAME;
pub const NAME: &str = "trigram";
pub const SHORT: &str = "t";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "TRIGRAM NUMBER";

pub fn declare_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(NAME)
        .short(SHORT)
        .long(LONG)
        .value_name(VALUE_NAME)
        .help(HELP)
        .long_help(LONG_HELP)
        .conflicts_with_all(&[question::NAME, hexagram::NAME, cast::NAME])
        .takes_value(TAKES_VALUE)
}
