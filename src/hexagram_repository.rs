use crate::hexagram::HexagramOrdering;
use crate::trigram::TrigramName;
use serde_derive::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
pub struct LineMeaning {
    pub position: String,
    pub meaning: String,
}

pub trait HexagramInfo {
    fn get_chinese_name(&self) -> &str;
    fn get_english_name(&self) -> &str;
    fn get_images(&self) -> &str;
    fn get_judgement(&self) -> &str;
    fn get_line_meanings(&self) -> &Vec<LineMeaning>;
    fn get_number(&self) -> i32;
    fn get_symbol(&self) -> &str;
    fn get_trigram_above_name(&self) -> TrigramName;
    fn get_trigram_below_name(&self) -> TrigramName;
}

pub trait HexagramRepository<InfoType: HexagramInfo + ?Sized> {
    fn get_by_number(&self, number: usize) -> Option<&InfoType>;
    fn get_is_initialized(&self) -> bool;
    fn get_ordering(&self) -> &HexagramOrdering;
    fn initialize(&mut self) -> Result<(), Box<Error>>;
    fn new() -> Self;
}
