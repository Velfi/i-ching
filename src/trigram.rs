use crate::hexagram::Hexagram;
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

#[derive(Deserialize)]
pub struct TrigramPair {
    above: Trigram,
    below: Trigram,
}

#[derive(Deserialize)]
pub struct RawTrigramPair {
    above: usize,
    below: usize,
}

impl TrigramPair {
    pub fn new(above: Trigram, below: Trigram) -> Self {
        TrigramPair {
            above,
            below,
        }
    }

    pub fn from_raw_trigram_pair(raw_trigram_pair: RawTrigramPair) -> Result<Self, TrigramError> {
        let above = Trigram::from_usize(raw_trigram_pair.above)?;
        let below = Trigram::from_usize(raw_trigram_pair.below)?;

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
            (Qian, Kun) => 1,
            (Qian, Zhen) => 1,
            (Qian, Kan) => 1,
            (Qian, Gen) => 1,
            (Qian, Xun) => 1,
            (Qian, Li) => 1,
            (Qian, Dui) => 1,
            (Kun, Qian) => 1,
            (Kun, Kun) => 1,
            (Kun, Zhen) => 1,
            (Kun, Kan) => 1,
            (Kun, Gen) => 1,
            (Kun, Xun) => 1,
            (Kun, Li) => 1,
            (Kun, Dui) => 1,
            (Zhen, Qian) => 1,
            (Zhen, Kun) => 1,
            (Zhen, Zhen) => 1,
            (Zhen, Kan) => 1,
            (Zhen, Gen) => 1,
            (Zhen, Xun) => 1,
            (Zhen, Li) => 1,
            (Zhen, Dui) => 1,
            (Kan, Qian) => 1,
            (Kan, Kun) => 1,
            (Kan, Zhen) => 1,
            (Kan, Kan) => 1,
            (Kan, Gen) => 1,
            (Kan, Xun) => 1,
            (Kan, Li) => 1,
            (Kan, Dui) => 1,
            (Gen, Qian) => 1,
            (Gen, Kun) => 1,
            (Gen, Zhen) => 1,
            (Gen, Kan) => 1,
            (Gen, Gen) => 1,
            (Gen, Xun) => 1,
            (Gen, Li) => 1,
            (Gen, Dui) => 1,
            (Xun, Qian) => 1,
            (Xun, Kun) => 1,
            (Xun, Zhen) => 1,
            (Xun, Kan) => 1,
            (Xun, Gen) => 1,
            (Xun, Xun) => 1,
            (Xun, Li) => 1,
            (Xun, Dui) => 1,
            (Li, Qian) => 1,
            (Li, Kun) => 1,
            (Li, Zhen) => 1,
            (Li, Kan) => 1,
            (Li, Gen) => 1,
            (Li, Xun) => 1,
            (Li, Li) => 1,
            (Li, Dui) => 1,
            (Dui, Qian) => 1,
            (Dui, Kun) => 1,
            (Dui, Zhen) => 1,
            (Dui, Kan) => 1,
            (Dui, Gen) => 1,
            (Dui, Xun) => 1,
            (Dui, Li) => 1,
            (Dui, Dui) => 1,
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
