use clap::Arg;

use crate::args::{cast, question, trigram};

pub const HELP: &str =
    "(A number 1-64){n}Look up a hexagram by number in the King Wen sequence.{n} ";
pub const LONG_HELP: &str = "\
Look up a hexagram by number in the King Wen sequence.{n}
example:{n}iching -x 42{n} ";
pub const LONG: &str = NAME;
pub const NAME: &str = "hexagram";
pub const SHORT: &str = "x";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "HEXAGRAM NUMBER";

pub fn declare_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(NAME)
        .short(SHORT)
        .long(LONG)
        .value_name(VALUE_NAME)
        .help(HELP)
        .long_help(LONG_HELP)
        .conflicts_with_all(&[question::NAME, trigram::NAME, cast::NAME])
        .takes_value(TAKES_VALUE)
}
