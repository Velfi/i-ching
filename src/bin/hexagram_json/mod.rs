use serde_derive::Deserialize;

use iching::{
    hexagram::HexagramOrdering,
    hexagram_repository::{
        ChangingLineMeaning,
        HexagramInfo,
        HexagramRepository,
    },
    trigram::{
        TrigramName,
        TrigramNamePair
    }
};

pub struct HexagramJson {
    _ordering: HexagramOrdering,
    _list: Vec<HexagramJsonInfo>,
    _is_initialized: bool,
}

impl HexagramJson {
    pub fn new() -> Self {
        HexagramJson::default()
    }
}

impl HexagramRepository for HexagramJson {
    fn get_by_number(&self, number: usize) -> Option<&dyn HexagramInfo> {
        if !self._is_initialized {
            panic!("Called 'get_by_number' on an uninitialized HexagramBook. Don't forget to initialize the book first!")
        }

        if let Some(actual_index) = number.checked_sub(1) {
            if let Some(hexagram_info) = self._list.get(actual_index) {
                return Some(hexagram_info as &dyn HexagramInfo);
            }
        }

        None
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
    number: usize,
    name: NameTranslations,
    #[serde(rename = "trigramPair")]
    trigram_usize_pair: TrigramUsizePair,
    judgement: String,
    images: String,
    lines: Vec<ChangingLineMeaning>,
}

#[derive(Deserialize)]
pub struct NameTranslations {
    english: String,
    chinese: String,
}

pub struct HexagramJsonInfo {
    _images: String,
    _judgement: String,
    _lines: Vec<ChangingLineMeaning>,
    _name: NameTranslations,
    _number: usize,
    _trigram_usize_pair: TrigramUsizePair,
}

impl HexagramInfo for HexagramJsonInfo {
    fn get_chinese_name(&self) -> &str {
        &self._name.chinese
    }

    fn get_localized_name(&self) -> &str {
        &self._name.english
    }

    fn get_images(&self) -> &str {
        &self._images
    }

    fn get_judgement(&self) -> &str {
        &self._judgement
    }

    fn get_line_meanings(&self, changing_lines: &[usize]) -> Vec<&ChangingLineMeaning> {
        self._lines
            .iter()
            .filter(|&line_meaning| changing_lines.contains(&line_meaning.position))
            .collect()
    }

    fn get_number(&self) -> usize {
        self._number
    }

    fn get_symbol(&self) -> &str {
        let above = TrigramName::from_usize(self._trigram_usize_pair.above).unwrap();
        let below = TrigramName::from_usize(self._trigram_usize_pair.below).unwrap();

        TrigramNamePair(above, below).as_symbol()
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
