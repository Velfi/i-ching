use crate::line::LineMeaning;
use crate::trigram::RawTrigramPair;
use crate::trigram::TrigramPair;
use serde_derive::Deserialize;
use std::fmt::{
    self,
    Display,
    Formatter,
};

#[derive(Deserialize)]
pub struct RawHexagram {
    number: i32,
    name: NameTranslations,
    #[serde(rename = "trigramPair")]
    raw_trigram_pair: RawTrigramPair,
    judgement: String,
    images: String,
    lines: Vec<LineMeaning>,
}

impl Default for RawHexagram {
    fn default() -> Self {
        unimplemented!()
    }
}

#[derive(Deserialize)]
pub struct NameTranslations {
    english: String,
    chinese: String,
}

pub struct Hexagram {
    _images: String,
    _judgement: String,
    _lines: Vec<LineMeaning>,
    _name: NameTranslations,
    _number: i32,
    _trigram_pair: TrigramPair,
}

impl Hexagram {
    pub fn english_name(&self) -> &str {
        &self._name.english
    }

    pub fn chinese_name(&self) -> &str {
        &self._name.chinese
    }
}

impl Display for Hexagram {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "  {} (No. {})", self._trigram_pair.symbol(), self._number);
        writeln!(f, "  {} - {}", self.chinese_name(), self.english_name());
        writeln!(f, "");
        writeln!(f, "  Judgement:");
        writeln!(f, "{}", self._judgement);
        writeln!(f, "  Images:");
        writeln!(f, "{}", self._images);

        Ok(())
    }
}

impl From<RawHexagram> for Hexagram {
    fn from(raw_hexagram: RawHexagram) -> Self {
        Hexagram {
            _number: raw_hexagram.number,
            _name: raw_hexagram.name,
            // An unwrap is safe here because it is assumed that this is only called by `Hexagrams`
            // during initialization.
            _trigram_pair: TrigramPair::from_raw_trigram_pair(raw_hexagram.raw_trigram_pair).unwrap(),
            _judgement: raw_hexagram.judgement,
            _images: raw_hexagram.images,
            _lines: raw_hexagram.lines,
        }
    }
}