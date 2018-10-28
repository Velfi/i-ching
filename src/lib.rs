pub mod coin;
pub mod line;
pub mod hexagram;
pub mod trigram;

use serde_json;
use std::fs::File;
use std::io::Read;
use crate::hexagram::RawHexagram;
use crate::hexagram::Hexagram;

pub enum Ordering {
    KingWen
}

pub struct Hexagrams {
    ordering: Ordering,
    list: Vec<Hexagram>,
    is_initialized: bool,
}

impl Hexagrams {
    pub fn new() -> Self {
        Hexagrams::default()
    }

    pub fn initialize(&mut self, hexagrams_file_path: &str) -> Result<(), serde_json::Error> {
        let mut f = File::open(hexagrams_file_path).expect("file not found");
//        let mut json_file: String = String::new();
//        f.read_to_string(&mut json_file)
//            .expect("something went wrong reading the file");
        let json_file = include_str!("hexagrams.json");
        let raw_hexagrams: Vec<RawHexagram> = serde_json::from_str(&json_file)?;
        self.list = raw_hexagrams.into_iter().map(|raw_hexagram| raw_hexagram.into()).collect();
        Ok(())
    }

    pub fn get_by_number(&self, number: usize) -> Option<&Hexagram> {
        match  number.checked_sub(1) {
            Some(actual_index) => self.list.get(actual_index),
            None => None
        }
    }
}

impl Default for Hexagrams {
    fn default() -> Self {
        Self {
            ordering: Ordering::KingWen,
            list: Vec::new(),
            is_initialized: false,
        }
    }
}
