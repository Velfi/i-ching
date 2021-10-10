use crate::args::{cast, question, trigram};
use clap::Arg;

pub const ABOUT: &str =
    "(A number 1-64){n}Look up a hexagram by number in the King Wen sequence.{n} ";
pub const LONG_ABOUT: &str = "\
Look up a hexagram by number in the King Wen sequence.{n}
example:{n}iching -x 42{n} ";
pub const LONG: &str = NAME;
pub const NAME: &str = "hexagram";
pub const SHORT: char = 'x';
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "HEXAGRAM NUMBER";

pub fn declare_arg<'a>() -> Arg<'a> {
    Arg::new(NAME)
        .short(SHORT)
        .long(LONG)
        .value_name(VALUE_NAME)
        .about(ABOUT)
        .long_about(LONG_ABOUT)
        .conflicts_with_all(&[question::NAME, trigram::NAME, cast::NAME])
        .takes_value(TAKES_VALUE)
}
