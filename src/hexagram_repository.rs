use std::{
    error::Error,
    fmt::{
        self,
        Display,
        Formatter,
    },
};

use serde_derive::Deserialize;

use crate::{
    hexagram::HexagramOrdering,
    trigram::TrigramName,
};

#[derive(Deserialize)]
pub struct LineMeaning {
    pub position: String,
    pub meaning: String,
}

pub trait HexagramInfo {
    fn get_chinese_name(&self) -> &str;
    fn get_localized_name(&self) -> &str;
    fn get_images(&self) -> &str;
    fn get_judgement(&self) -> &str;
    fn get_line_meanings(&self) -> &Vec<LineMeaning>;
    fn get_number(&self) -> i32;
    fn get_symbol(&self) -> &str;
    fn get_trigram_above_name(&self) -> TrigramName;
    fn get_trigram_below_name(&self) -> TrigramName;
}

pub trait HexagramRepository {
    fn get_by_number(&self, number: usize) -> Option<&dyn HexagramInfo>;
    fn get_is_initialized(&self) -> bool;
    fn get_ordering(&self) -> &HexagramOrdering;
    fn initialize(&mut self) -> Result<(), Box<Error>>;
    fn new() -> Self;
}

impl Display for &dyn HexagramInfo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "  {} (No. {})", self.get_symbol(), self.get_number());
        writeln!(f, "  {} - {}", self.get_chinese_name(), self.get_localized_name());
        writeln!(f);
        writeln!(f, "  Judgement:");
        writeln!(f, "{}", self.get_judgement());
        writeln!(f, "  Images:");
        writeln!(f, "{}", self.get_images());

        Ok(())
    }
}