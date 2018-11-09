use clap::Arg;

pub const HELP: &str = "(A number 1-64) Look up a hexagram by number (King Wen sequence)";
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
//        .conflicts_with_all(&[ARG_QUESTION.name, ARG_TRIGRAM.name, ARG_CAST.name])
        .takes_value(TAKES_VALUE)
}