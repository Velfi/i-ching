use clap::Arg;

pub const HELP: &str = "Cast your own coins and enter the sequence as a series of digits";
pub const LONG_HELP: &str = "derp";
pub const LONG: &str = NAME;
pub const NAME: &str = "cast";
pub const SHORT: &str = "c";
pub const TAKES_VALUE: bool = true;
pub const VALUE_NAME: &str = "COIN TOSS RESULTS";

pub fn declare_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(NAME)
        .short(SHORT)
        .long(LONG)
        .value_name(VALUE_NAME)
        .help(HELP)
        .takes_value(TAKES_VALUE)
}