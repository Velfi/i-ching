use crate::line::Line;
use serde_derive::Deserialize;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;

#[derive(Deserialize)]
pub enum Trigram {
    Qian,
    Kun,
    Zhen,
    Kan,
    Gen,
    Xun,
    Li,
    Dui,
}

struct TrigramLines(Line, Line, Line);

impl Default for TrigramLines {
    fn default() -> Self {
        TrigramLines(
            rand::random(),
            rand::random(),
            rand::random(),
        )
    }
}

impl Trigram {
    pub fn symbol(&self) -> String {
        use self::Trigram::*;
        match self {
            Qian => String::from("☰"),
            Kun => String::from("☷"),
            Zhen => String::from("☳"),
            Kan => String::from("☵"),
            Gen => String::from("☶"),
            Xun => String::from("☴"),
            Li => String::from("☲"),
            Dui => String::from("☱"),
        }
    }

    pub fn romanized(&self) -> String {
        use self::Trigram::*;
        match self {
            Qian => String::from("Qián"),
            Kun => String::from("Kūn"),
            Zhen => String::from("Zhèn"),
            Kan => String::from("Kǎn"),
            Gen => String::from("Gèn"),
            Xun => String::from("Xùn"),
            Li => String::from("Lí"),
            Dui => String::from("Duì"),
        }
    }

    pub fn english_translation(&self) -> String {
        use self::Trigram::*;
        match self {
            Qian => String::from("The Creative"),
            Kun => String::from("The Receptive"),
            Zhen => String::from("The Arousing"),
            Kan => String::from("The Abysmal"),
            Gen => String::from("Keeping Still"),
            Xun => String::from("The Gentle"),
            Li => String::from("The Clinging"),
            Dui => String::from("The Joyous"),
        }
    }

    pub fn attribute(&self) -> String {
        use self::Trigram::*;
        match self {
            Qian => String::from("strong"),
            Kun => String::from("devoted, yielding"),
            Zhen => String::from("inciting, movement"),
            Kan => String::from("dangerous"),
            Gen => String::from("resting"),
            Xun => String::from("penetrating"),
            Li => String::from("light-giving"),
            Dui => String::from("joyful"),
        }
    }

    pub fn image(&self) -> String {
        use self::Trigram::*;
        match self {
            Qian => String::from("heaven"),
            Kun => String::from("earth"),
            Zhen => String::from("thunder"),
            Kan => String::from("water"),
            Gen => String::from("mountain"),
            Xun => String::from("wind, wood"),
            Li => String::from("fire"),
            Dui => String::from("lake"),
        }
    }

    pub fn family_relationship(&self) -> String {
        use self::Trigram::*;
        match self {
            Qian => String::from("father"),
            Kun => String::from("mother"),
            Zhen => String::from("first son"),
            Kan => String::from("second son"),
            Gen => String::from("third son"),
            Xun => String::from("first daughter"),
            Li => String::from("second daughter"),
            Dui => String::from("third daughter"),
        }
    }

    pub fn from_usize(number: usize) -> Result<Self, TrigramError> {
        use self::Trigram::*;
        match number {
            1 => Ok(Qian),
            2 => Ok(Dui),
            3 => Ok(Li),
            4 => Ok(Zhen),
            5 => Ok(Xun),
            6 => Ok(Kan),
            7 => Ok(Gen),
            8 => Ok(Kun),
            _ => Err(TrigramError::IntegerOutOfRange)
        }
    }

    pub fn number(&self) -> usize {
        use self::Trigram::*;
        match self {
            Qian => 1,
            Dui => 2,
            Li => 3,
            Zhen => 4,
            Xun => 5,
            Kan => 6,
            Gen => 7,
            Kun => 8,
        }
    }

    pub fn from_lines(lines: RawTrigram) -> Trigram {
        Trigram::_from_lines(lines, false)
    }

    pub fn from_changing_lines(lines: RawTrigram) -> Trigram {
        Trigram::_from_lines(lines, true)
    }

    pub fn _from_lines(lines: RawTrigram, with_changes: bool) -> Trigram {
        use self::Trigram::*;
        use crate::line::Line::*;

        let l1 = lines.0.settle(with_changes);
        let l2 = lines.1.settle(with_changes);
        let l3 = lines.2.settle(with_changes);

        match (l1, l2, l3) {
            (Unbroken, Unbroken, Unbroken) => Qian,// "☰",
            (Broken, Broken, Broken) => Kun,// "☷",
            (Broken, Broken, Unbroken) => Zhen,// "☳",
            (Broken, Unbroken, Broken) => Kan,// "☵",
            (Unbroken, Broken, Broken) => Gen,// "☶",
            (Unbroken, Unbroken, Broken) => Xun,// "☴",
            (Unbroken, Broken, Unbroken) => Li,// "☲",
            (Broken, Unbroken, Unbroken) => Dui,// "☱",
            // Per the settling above, this is unreachable
            (_, _, _) => unreachable!()
        }
    }
}

impl Display for Trigram {
    fn fmt(&self, f: &mut Formatter<>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{} (No. {})", self.symbol(), self.number());
        writeln!(f, "{} - {}", self.romanized(), self.english_translation());
        writeln!(f, "");
        writeln!(f, "Attribute: {}", self.attribute());
        writeln!(f, "Image in nature: {}", self.image());
        writeln!(f, "Family Relationship: {}", self.family_relationship());

        Ok(())
    }
}

pub struct TrigramPair {
    above: Trigram,
    below: Trigram,
}

#[derive(Deserialize)]
pub struct TrigramUsizePair {
    above: usize,
    below: usize,
}

pub struct TrigramLinesPair {
    above: TrigramLines,
    below: TrigramLines,
}

impl TrigramPair {
    pub fn new(above: Trigram, below: Trigram) -> Self {
        TrigramPair {
            above,
            below,
        }
    }

    pub fn from_trigram_usize_pair(trigram_usize_pair: TrigramUsizePair) -> Result<Self, TrigramError> {
        let above = Trigram::from_usize(trigram_usize_pair.above)?;
        let below = Trigram::from_usize(trigram_usize_pair.below)?;

        Ok(TrigramPair {
            above,
            below,
        })
    }

    pub fn symbol(&self) -> String {
        use self::Trigram::*;
        match (&self.above, &self.below) {
            (Qian, Qian) => String::from("䷀"),
            (Qian, Kun) => String::from("䷋"),
            (Qian, Zhen) => String::from("䷘"),
            (Qian, Kan) => String::from("䷅"),
            (Qian, Gen) => String::from("䷠"),
            (Qian, Xun) => String::from("䷫"),
            (Qian, Li) => String::from("䷌"),
            (Qian, Dui) => String::from("䷉"),
            (Kun, Qian) => String::from("䷊"),
            (Kun, Kun) => String::from("䷁"),
            (Kun, Zhen) => String::from("䷗"),
            (Kun, Kan) => String::from("䷆"),
            (Kun, Gen) => String::from("䷎"),
            (Kun, Xun) => String::from("䷭"),
            (Kun, Li) => String::from("䷣"),
            (Kun, Dui) => String::from("䷒"),
            (Zhen, Qian) => String::from("䷡"),
            (Zhen, Kun) => String::from("䷏"),
            (Zhen, Zhen) => String::from("䷲"),
            (Zhen, Kan) => String::from("䷧"),
            (Zhen, Gen) => String::from("䷽"),
            (Zhen, Xun) => String::from("䷟"),
            (Zhen, Li) => String::from("䷶"),
            (Zhen, Dui) => String::from("䷵"),
            (Kan, Qian) => String::from("䷄"),
            (Kan, Kun) => String::from("䷇"),
            (Kan, Zhen) => String::from("䷂"),
            (Kan, Kan) => String::from("䷜"),
            (Kan, Gen) => String::from("䷦"),
            (Kan, Xun) => String::from("䷯"),
            (Kan, Li) => String::from("䷾"),
            (Kan, Dui) => String::from("䷻"),
            (Gen, Qian) => String::from("䷙"),
            (Gen, Kun) => String::from("䷖"),
            (Gen, Zhen) => String::from("䷚"),
            (Gen, Kan) => String::from("䷃"),
            (Gen, Gen) => String::from("䷳"),
            (Gen, Xun) => String::from("䷑"),
            (Gen, Li) => String::from("䷕"),
            (Gen, Dui) => String::from("䷨"),
            (Xun, Qian) => String::from("䷈"),
            (Xun, Kun) => String::from("䷓"),
            (Xun, Zhen) => String::from("䷩"),
            (Xun, Kan) => String::from("䷺"),
            (Xun, Gen) => String::from("䷴"),
            (Xun, Xun) => String::from("䷸"),
            (Xun, Li) => String::from("䷤"),
            (Xun, Dui) => String::from("䷼"),
            (Li, Qian) => String::from("䷍"),
            (Li, Kun) => String::from("䷢"),
            (Li, Zhen) => String::from("䷔"),
            (Li, Kan) => String::from("䷿"),
            (Li, Gen) => String::from("䷷"),
            (Li, Xun) => String::from("䷱"),
            (Li, Li) => String::from("䷝"),
            (Li, Dui) => String::from("䷥"),
            (Dui, Qian) => String::from("䷪"),
            (Dui, Kun) => String::from("䷬"),
            (Dui, Zhen) => String::from("䷐"),
            (Dui, Kan) => String::from("䷮"),
            (Dui, Gen) => String::from("䷞"),
            (Dui, Xun) => String::from("䷛"),
            (Dui, Li) => String::from("䷰"),
            (Dui, Dui) => String::from("䷹"),
        }
    }

    pub fn king_wen_sequence(&self) -> usize {
        use self::Trigram::*;
        match (&self.above, &self.below) {
            (Qian, Qian) => 1,
            (Qian, Kun) => 12,
            (Qian, Zhen) => 25,
            (Qian, Kan) => 6,
            (Qian, Gen) => 33,
            (Qian, Xun) => 44,
            (Qian, Li) => 13,
            (Qian, Dui) => 10,
            (Kun, Qian) => 11,
            (Kun, Kun) => 2,
            (Kun, Zhen) => 24,
            (Kun, Kan) => 7,
            (Kun, Gen) => 15,
            (Kun, Xun) => 46,
            (Kun, Li) => 36,
            (Kun, Dui) => 19,
            (Zhen, Qian) => 34,
            (Zhen, Kun) => 16,
            (Zhen, Zhen) => 51,
            (Zhen, Kan) => 40,
            (Zhen, Gen) => 62,
            (Zhen, Xun) => 32,
            (Zhen, Li) => 55,
            (Zhen, Dui) => 54,
            (Kan, Qian) => 5,
            (Kan, Kun) => 8,
            (Kan, Zhen) => 3,
            (Kan, Kan) => 29,
            (Kan, Gen) => 39,
            (Kan, Xun) => 48,
            (Kan, Li) => 63,
            (Kan, Dui) => 60,
            (Gen, Qian) => 26,
            (Gen, Kun) => 23,
            (Gen, Zhen) => 27,
            (Gen, Kan) => 4,
            (Gen, Gen) => 52,
            (Gen, Xun) => 18,
            (Gen, Li) => 22,
            (Gen, Dui) => 41,
            (Xun, Qian) => 9,
            (Xun, Kun) => 20,
            (Xun, Zhen) => 42,
            (Xun, Kan) => 59,
            (Xun, Gen) => 53,
            (Xun, Xun) => 57,
            (Xun, Li) => 37,
            (Xun, Dui) => 61,
            (Li, Qian) => 14,
            (Li, Kun) => 35,
            (Li, Zhen) => 21,
            (Li, Kan) => 64,
            (Li, Gen) => 56,
            (Li, Xun) => 50,
            (Li, Li) => 30,
            (Li, Dui) => 38,
            (Dui, Qian) => 43,
            (Dui, Kun) => 45,
            (Dui, Zhen) => 17,
            (Dui, Kan) => 47,
            (Dui, Gen) => 31,
            (Dui, Xun) => 28,
            (Dui, Li) => 49,
            (Dui, Dui) => 58,
        }
    }
}

impl Add for Trigram {
    type Output = TrigramPair;

    fn add(self, other: Trigram) -> TrigramPair {
        TrigramPair::new(self, other)
    }
}

#[derive(Debug)]
pub enum TrigramError {
    IntegerOutOfRange,
    ConversionFailed,
}

static INTEGER_OUT_OF_RANGE_ERROR_MSG: &str = "Invalid conversion from i32 to Trigram, make sure your i32 is between 1-8 inclusive";
static RAW_TRIGRAM_PAIR_TO_TRIGRAM_PAIR_CONVERSION_ERROR_MSG: &str = "Failed to convert RawTrigramPair into TrigramPair";

impl Display for TrigramError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        use self::TrigramError::*;
        match self {
            IntegerOutOfRange => write!(f, "{}", INTEGER_OUT_OF_RANGE_ERROR_MSG),
            ConversionFailed => write!(f, "{}", RAW_TRIGRAM_PAIR_TO_TRIGRAM_PAIR_CONVERSION_ERROR_MSG),
        }
    }
}

impl Error for TrigramError {
    fn description(&self) -> &str {
        use self::TrigramError::*;
        match self {
            IntegerOutOfRange => INTEGER_OUT_OF_RANGE_ERROR_MSG,
            ConversionFailed => RAW_TRIGRAM_PAIR_TO_TRIGRAM_PAIR_CONVERSION_ERROR_MSG,
        }
    }
}
