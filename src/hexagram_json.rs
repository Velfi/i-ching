use iching::{
    hexagram::{Hexagram, HexagramOrdering},
    hexagram_repository::HexagramRepository,
    trigram::Trigram,
};
use serde_derive::Deserialize;
use std::io::Write;
use termcolor::{Color, ColorSpec, WriteColor};

pub(crate) struct HexagramJson {
    ordering: HexagramOrdering,
    list: Vec<HexagramJsonInfo>,
    is_initialized: bool,
}

impl HexagramJson {
    pub(crate) fn new() -> Self {
        HexagramJson::default()
    }
}

impl HexagramRepository for HexagramJson {
    type HexagramInfo = HexagramJsonInfo;

    fn get_by_number(&self, number: u8) -> Option<&Self::HexagramInfo> {
        if !self.is_initialized {
            panic!("Called 'get_by_number' on an uninitialized HexagramJson. Don't forget to initialize the repository first!")
        }

        if let Some(actual_index) = number.checked_sub(1) {
            if let Some(hexagram_info) = self.list.get(actual_index as usize) {
                return Some(hexagram_info as &Self::HexagramInfo);
            }
        }

        None
    }

    fn get_info_for_hexagram(&self, hexagram: &Hexagram) -> &Self::HexagramInfo {
        if !self.is_initialized {
            panic!("Called 'get_by_hexagram' on an uninitialized HexagramJson. Don't forget to initialize the repository first!")
        }

        self.list
            .iter()
            .find(|&hexagram_info| {
                hexagram.above().number() == hexagram_info.trigrams.above.number()
                    && hexagram.below().number() == hexagram_info.trigrams.below.number()
            })
            .expect("HexagramJson contains all possible hexagrams.")
    }

    fn get_is_initialized(&self) -> bool {
        self.is_initialized
    }

    fn get_ordering(&self) -> &HexagramOrdering {
        &self.ordering
    }

    // Load hexagram data from json. The json file is inlined into the executable using
    // `include_str!`. Then, `serde_json` parses it into a `Vec<RawHexagramInfo>` which
    // is then converted into a `Vec<HexagramInfo>`.
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let json_file = include_str!("hexagrams.json");
        let raw_hexagram_json: Vec<RawHexagramJsonInfo> = serde_json::from_str(json_file)?;
        self.list = raw_hexagram_json
            .into_iter()
            .map(|raw_hexagram_json| raw_hexagram_json.into())
            .collect();

        self.is_initialized = true;
        Ok(())
    }
}

impl Default for HexagramJson {
    fn default() -> Self {
        Self {
            ordering: HexagramOrdering::KingWen,
            list: Vec::new(),
            is_initialized: false,
        }
    }
}

#[derive(Deserialize)]
struct RawHexagramJsonInfo {
    number: usize,
    name: NameTranslations,
    trigrams: RawTrigrams,
    judgement: String,
    images: String,
    lines: Vec<ChangingLineMeaning>,
}

/// Associates the meaning of a changing line with the position of a changing line in a hexagram.
#[derive(Deserialize)]
pub(crate) struct ChangingLineMeaning {
    pub(crate) position: usize,
    pub(crate) meaning: String,
}

#[derive(Deserialize)]
struct NameTranslations {
    english: String,
    chinese: String,
    pinyin: String,
}

#[derive(Deserialize)]
struct RawTrigrams {
    above: usize,
    below: usize,
}

pub(crate) struct HexagramJsonInfo {
    images: String,
    judgement: String,
    lines: Vec<ChangingLineMeaning>,
    name: NameTranslations,
    number: usize,
    trigrams: Trigrams,
    hexagram: Hexagram,
}

struct Trigrams {
    above: Trigram,
    below: Trigram,
}

impl HexagramJsonInfo {
    pub(crate) fn line_meanings(&self, changing_lines: &[usize]) -> Vec<&ChangingLineMeaning> {
        self.lines
            .iter()
            .filter(|&line_meaning| changing_lines.contains(&(line_meaning.position)))
            .collect()
    }

    pub(crate) fn write_to<T>(&self, output: &mut T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: WriteColor + Write,
    {
        // Write out header
        output
            .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(196))))
            .expect("output stream color can be set");
        writeln!(
            output,
            "Hexagram No. {}  {}",
            self.number,
            self.hexagram.symbol(false)
        )?;
        output.reset().expect("output stream color can be reset");

        writeln!(output, "\t{}", self.name.english)?;
        writeln!(output, "\t{} ({})", self.name.chinese, self.name.pinyin)?;
        writeln!(output)?;

        // Write out judgement
        output
            .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(160))))
            .expect("output stream color can be set");
        writeln!(output, "Judgement:")?;
        output.reset().expect("output stream color can be reset");
        writeln!(output, "\t{}", self.judgement)?;

        // Write out images
        output
            .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(124))))
            .expect("output stream color can be set");
        writeln!(output, "Images:")?;
        output.reset().expect("output stream color can be reset");
        writeln!(output, "\t{}", self.images)?;

        Ok(())
    }
}

impl From<RawHexagramJsonInfo> for HexagramJsonInfo {
    fn from(
        RawHexagramJsonInfo {
            number,
            name,
            trigrams,
            images,
            lines,
            judgement,
        }: RawHexagramJsonInfo,
    ) -> Self {
        // Add a tab to the beginning of each line in the judgement and images. This'll improve the
        // formatting of the text when it's displayed in the terminal.
        let judgement = judgement.replace('\n', "\n\t");
        let images = images.replace('\n', "\n\t");

        let trigrams = Trigrams {
            above: trigrams
                .above
                .try_into()
                .expect("trigram numbers in JSON file are valid"),
            below: trigrams
                .below
                .try_into()
                .expect("trigram numbers in JSON file are valid"),
        };
        let hexagram = Hexagram::new(trigrams.above, trigrams.below);

        HexagramJsonInfo {
            number,
            name,
            trigrams,
            judgement,
            images,
            lines,
            hexagram,
        }
    }
}

#[cfg(test)]
mod tests {
    use iching::HexagramRepository;
    use termcolor::Buffer;

    #[test]
    fn test_writing_without_color_works() {
        let mut output = Buffer::no_color();
        let mut hexagrams = super::HexagramJson::default();
        hexagrams.initialize().unwrap();
        let hexagram = hexagrams.get_by_number(1).unwrap();
        hexagram.write_to(&mut output).unwrap();

        let output = String::from_utf8(output.into_inner()).unwrap();

        assert_eq!(
            output,
            "Hexagram No. 1  ䷀\n\tThe Creative\n\t乾 (Qián)\n\nJudgement:\n\tThe Creative works sublime success,\n\tFurthering through perseverance.\nImages:\n\tThe movement of heaven is full of power.\n\tThus the superior man makes himself strong and untiring.\n"
        );
    }

    #[test]
    fn test_writing_with_color_works() {
        let mut output = Buffer::ansi();
        let mut hexagrams = super::HexagramJson::default();
        hexagrams.initialize().unwrap();
        let hexagram = hexagrams.get_by_number(1).unwrap();
        hexagram.write_to(&mut output).unwrap();

        let output = String::from_utf8(output.into_inner()).unwrap();

        assert_eq!(
            output,
            "\u{1b}[0m\u{1b}[38;5;196mHexagram No. 1  ䷀\n\u{1b}[0m\tThe Creative\n\t乾 (Qián)\n\n\u{1b}[0m\u{1b}[38;5;160mJudgement:\n\u{1b}[0m\tThe Creative works sublime success,\n\tFurthering through perseverance.\n\u{1b}[0m\u{1b}[38;5;124mImages:\n\u{1b}[0m\tThe movement of heaven is full of power.\n\tThus the superior man makes himself strong and untiring.\n"
        );
    }
}
