use clap::Arg;

use crate::args::{
    hexagram, trigram,
};

pub const HELP: &str = "(A double-quoted string){n}A question that you wish to answer.{n} ";
pub const LONG_HELP: &str = "\
This should be a question that you want to try to answer with divination. \
General questions are better than specific questions. Divination is an \
indirect method of introspection, and can help you become conscious of \
how you feel about something. With that in mind, take time to think about \
what you want to decide by consulting the I Ching. Your question should be a \
double quoted string.{n}
example:{n}iching -q \"What do I need to know going forward?\"{n} ";
pub const LONG: &str = NAME;
pub const NAME: &str = "question";
pub const SHORT: &str = "q";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "QUESTION";

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