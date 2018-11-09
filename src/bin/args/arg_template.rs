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