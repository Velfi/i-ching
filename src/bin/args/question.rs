use clap::Arg;

pub const HELP: &str = "(A double-quoted string) A question that you wish to answer";
pub const LONG: &str = NAME;
pub const NAME: &str = "question";
pub const SHORT: &str = "q";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "QUESTION";

pub fn declare_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(NAME)
        .short(SHORT)
        .long(LONG)
        .value_name(VALUE_NAME)
        .help(HELP)
//        .conflicts_with_all(&[ARG_TRIGRAM.name, ARG_HEXAGRAM.name])
        .takes_value(TAKES_VALUE)
}