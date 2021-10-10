use crate::args::{hexagram, trigram};
use clap::Arg;

pub const ABOUT: &str = "(A double-quoted string){n}A question that you wish to answer.{n} ";
pub const LONG_ABOUT: &str = "\
This should be a question that you want to try to answer with divination. \
General questions are better than specific questions. Divination is an \
indirect method of introspection, and can help you become conscious of \
how you feel about something. With that in mind, take time to think about \
what you want to decide by consulting the I Ching. Your question should be a \
double quoted string.{n}
example:{n}iching -q \"What do I need to know going forward?\"{n} ";
pub const LONG: &str = NAME;
pub const NAME: &str = "question";
pub const SHORT: char = 'q';
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "QUESTION";

pub fn declare_arg<'a>() -> Arg<'a> {
    Arg::new(NAME)
        .conflicts_with_all(&[trigram::NAME, hexagram::NAME])
        .short(SHORT)
        .long(LONG)
        .value_name(VALUE_NAME)
        .about(ABOUT)
        .long_about(LONG_ABOUT)
        .takes_value(TAKES_VALUE)
}
