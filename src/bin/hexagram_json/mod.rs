use iching::{
    hexagram::HexagramOrdering,
    hexagram_repository::{
        HexagramInfo,
        HexagramRepository,
        LineMeaning,
    },
    trigram::{
        trigram_name_pair_as_symbol,
        TrigramName,
    },
};
use serde_derive::Deserialize;
use std::fmt::{
    self,
    Display,
    Formatter,
};

pub struct HexagramJson {
    _ordering: HexagramOrdering,
    _list: Vec<HexagramJsonInfo>,
    _is_initialized: bool,
}

impl HexagramRepository<HexagramJsonInfo> for HexagramJson {
    fn get_by_number(&self, number: usize) -> Option<&HexagramJsonInfo> {
        if !self._is_initialized {
            panic!("Called 'get_by_number' on an uninitialized HexagramBook. Don't forget to initialize the book first!")
        }

        match number.checked_sub(1) {
            Some(actual_index) => self._list.get(actual_index),
            None => None
        }
    }

    fn get_is_initialized(&self) -> bool {
        self._is_initialized
    }

    fn get_ordering(&self) -> &HexagramOrdering {
        &self._ordering
    }

    // Load hexagram data from json. The json file is inlined into the executable using
    // `include_str!`. Then, `serde_json` parses it into a `Vec<RawHexagramInfo>` which
    // is then converted into a `Vec<HexagramInfo>`.
    fn initialize(&mut self) -> Result<(), Box<std::error::Error>> {
        let json_file = include_str!("hexagrams.json");
        let raw_hexagram_json: Vec<RawHexagramJsonInfo> = serde_json::from_str(&json_file)?;
        self._list = raw_hexagram_json.into_iter().map(|raw_hexagram_json| raw_hexagram_json.into()).collect();
        self._is_initialized = true;
        Ok(())
    }

    fn new() -> Self {
        HexagramJson::default()
    }
}

impl Default for HexagramJson {
    fn default() -> Self {
        Self {
            _ordering: HexagramOrdering::KingWen,
            _list: Vec::new(),
            _is_initialized: false,
        }
    }
}

#[derive(Deserialize)]
pub struct RawHexagramJsonInfo {
    number: i32,
    name: NameTranslations,
    #[serde(rename = "trigramPair")]
    trigram_usize_pair: TrigramUsizePair,
    judgement: String,
    images: String,
    lines: Vec<LineMeaning>,
}

#[derive(Deserialize)]
pub struct NameTranslations {
    english: String,
    chinese: String,
}

pub struct HexagramJsonInfo {
    _images: String,
    _judgement: String,
    _lines: Vec<LineMeaning>,
    _name: NameTranslations,
    _number: i32,
    _trigram_usize_pair: TrigramUsizePair,
}

impl HexagramInfo for HexagramJsonInfo {
    fn get_chinese_name(&self) -> &str {
        &self._name.chinese
    }

    fn get_english_name(&self) -> &str {
        &self._name.english
    }

    fn get_images(&self) -> &str {
        &self._images
    }

    fn get_judgement(&self) -> &str {
        &self._judgement
    }

    fn get_line_meanings(&self) -> &Vec<LineMeaning> {
        &self._lines
    }

    fn get_number(&self) -> i32 {
        self._number
    }

    fn get_symbol(&self) -> &str {
        let above = TrigramName::from_usize(self._trigram_usize_pair.above).unwrap();
        let below = TrigramName::from_usize(self._trigram_usize_pair.below).unwrap();

        trigram_name_pair_as_symbol(&(above, below))
    }

    fn get_trigram_above_name(&self) -> TrigramName {
        // A failed conversion here should be impossible because the JSON is valid.
        TrigramName::from_usize(self._trigram_usize_pair.above).unwrap()
    }
    fn get_trigram_below_name(&self) -> TrigramName {
        // A failed conversion here should be impossible because the JSON is valid.
        TrigramName::from_usize(self._trigram_usize_pair.below).unwrap()
    }
}

impl From<RawHexagramJsonInfo> for HexagramJsonInfo {
    fn from(raw_hexagram: RawHexagramJsonInfo) -> Self {
        HexagramJsonInfo {
            _number: raw_hexagram.number,
            _name: raw_hexagram.name,
            _trigram_usize_pair: raw_hexagram.trigram_usize_pair,
            _judgement: raw_hexagram.judgement,
            _images: raw_hexagram.images,
            _lines: raw_hexagram.lines,
        }
    }
}

#[derive(Deserialize)]
pub struct TrigramUsizePair {
    above: usize,
    below: usize,
}

impl Display for HexagramJsonInfo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "  {} (No. {})", self.get_symbol(), self.get_number());
        writeln!(f, "  {} - {}", self.get_chinese_name(), self.get_english_name());
        writeln!(f);
        writeln!(f, "  Judgement:");
        writeln!(f, "{}", self.get_judgement());
        writeln!(f, "  Images:");
        writeln!(f, "{}", self.get_images());

        Ok(())
    }
}