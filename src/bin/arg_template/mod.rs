pub struct ArgTemplate {
    pub help: &'static str,
    pub long: &'static str,
    pub name: &'static str,
    pub short: &'static str,
    pub takes_value: bool,
    pub value_name: &'static str,
}

impl AsRef<str> for ArgTemplate {
    fn as_ref(&self) -> &str {
        self.name
    }
}

pub const ARG_QUESTION: ArgTemplate = ArgTemplate {
    help: "(A double-quoted string) A question that you wish to answer",
    long: "question",
    name: "question",
    short: "q",
    takes_value: true,
    value_name: "QUESTION",
};

pub const ARG_HEXAGRAM: ArgTemplate = ArgTemplate {
    help: "(A number 1-64) Look up a hexagram by number (King Wen sequence)",
    long: "hexagram",
    name: "hexagram",
    short: "x",
    takes_value: true,
    value_name: "HEXAGRAM NUMBER",
};

pub const ARG_TRIGRAM: ArgTemplate = ArgTemplate {
    help: "(A number 1-8) Look up a trigram by number",
    long: "trigram",
    name: "trigram",
    short: "t",
    takes_value: true,
    value_name: "TRIGRAM NUMBER",
};

pub const ARG_CAST: ArgTemplate = ArgTemplate {
    help: "Cast your own coins and enter the sequence as a series of digits",
    long: "cast",
    name: "cast",
    short: "c",
    takes_value: true,
    value_name: "COIN TOSS RESULTS",
};
