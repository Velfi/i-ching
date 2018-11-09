use clap::Arg;

pub const HELP: &str = "(A number 1-8) Look up a trigram by number";
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
//        .conflicts_with_all(&[ARG_QUESTION.name, ARG_HEXAGRAM.name, ARG_CAST.name])
        .takes_value(TAKES_VALUE)
}