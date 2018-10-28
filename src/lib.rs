use crate::hexagram::{
    Hexagram,
    RawHexagram,
};
use crate::trigram::TrigramPair;
use serde_json;

pub mod coin;
pub mod line;
pub mod hexagram;
pub mod trigram;

pub enum Ordering {
    KingWen
}

pub struct Hexagrams {
    _ordering: Ordering,
    _list: Vec<Hexagram>,
    _is_initialized: bool,
}

impl Hexagrams {
    pub fn new() -> Self {
        Hexagrams::default()
    }

    // Load hexagram data from json. The json file is inlined into the executable using
    // `include_str!`. Then, `serde_json` parses it into a `Vec<Hexagram>`.
    pub fn initialize(&mut self) -> Result<(), serde_json::Error> {
        let json_file = include_str!("hexagrams.json");
        let raw_hexagrams: Vec<RawHexagram> = serde_json::from_str(&json_file)?;
        self._list = raw_hexagrams.into_iter().map(|raw_hexagram| raw_hexagram.into()).collect();
        Ok(())
    }

    pub fn get_by_number(&self, number: usize) -> Option<&Hexagram> {
        match number.checked_sub(1) {
            Some(actual_index) => self._list.get(actual_index),
            None => None
        }
    }

    pub fn get_ordering(&self) -> &Ordering {
        &self._ordering
    }

    pub fn get_is_initialized(&self) -> &bool {
        &self._is_initialized
    }

    pub fn divine_with_lines() -> (&Hexagram, Option<&Hexagram>) {
        above_raw_trigram = RawTrigram::default();
        below_raw_trigram = RawTrigram::default();

        trigram_pair_unchanged = RawTrigramPair::(above);
        trigram_pair_changed = RawTrigramPair::(above);
    }
}

impl Default for Hexagrams {
    fn default() -> Self {
        Self {
            _ordering: Ordering::KingWen,
            _list: Vec::new(),
            _is_initialized: false,
        }
    }
}
