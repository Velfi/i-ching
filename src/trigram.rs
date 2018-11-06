use crate::line::Line;
use crate::symbols::{
    hexagram::*,
    trigram::*,
};
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;

//#[derive(Deserialize)]
pub enum TrigramName {
    Qian,
    Kun,
    Zhen,
    Kan,
    Gen,
    Xun,
    Li,
    Dui,
}

impl TrigramName {
    pub fn symbol(&self) -> &str {
        use self::TrigramName::*;
        match self {
            Qian => QIAN_SYMBOL,
            Kun => KUN_SYMBOL,
            Zhen => ZHEN_SYMBOL,
            Kan => KAN_SYMBOL,
            Gen => GEN_SYMBOL,
            Xun => XUN_SYMBOL,
            Li => LI_SYMBOL,
            Dui => DUI_SYMBOL,
        }
    }

    pub fn romanized(&self) -> String {
        use self::TrigramName::*;
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
        use self::TrigramName::*;
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
        use self::TrigramName::*;
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
        use self::TrigramName::*;
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
        use self::TrigramName::*;
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
        use self::TrigramName::*;
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
        use self::TrigramName::*;
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

impl Add for TrigramName {
    type Output = usize;

    fn add(self, other: TrigramName) -> usize {
        self.number() + other.number()
    }
}

impl Display for TrigramName {
    fn fmt(&self, f: &mut Formatter<>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{} (No. {})", self.symbol(), self.number());
        writeln!(f, "{} - {}", self.romanized(), self.english_translation());
        writeln!(f);
        writeln!(f, "Attribute: {}", self.attribute());
        writeln!(f, "Image in nature: {}", self.image());
        writeln!(f, "Family Relationship: {}", self.family_relationship());

        Ok(())
    }
}

pub struct Trigram(Line, Line, Line);

impl Trigram {
    pub fn get_name(&self, with_changes: bool) -> TrigramName {
        use self::TrigramName::*;
        use crate::line::Line::*;

        let l1 = self.0.settle(with_changes);
        let l2 = self.1.settle(with_changes);
        let l3 = self.2.settle(with_changes);

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

impl Default for Trigram {
    fn default() -> Self {
        Trigram(
            rand::random(),
            rand::random(),
            rand::random(),
        )
    }
}

pub type TrigramNamePair = (TrigramName, TrigramName);

pub fn trigram_name_pair_as_symbol(trigram_name_pair: &TrigramNamePair) -> &'static str {
    use self::TrigramName::*;
    match trigram_name_pair {
        (Qian, Qian) => QIAN_QIAN_SYMBOL,
        (Qian, Kun) => QIAN_KUN_SYMBOL,
        (Qian, Zhen) => QIAN_ZHEN_SYMBOL,
        (Qian, Kan) => QIAN_KAN_SYMBOL,
        (Qian, Gen) => QIAN_GEN_SYMBOL,
        (Qian, Xun) => QIAN_XUN_SYMBOL,
        (Qian, Li) => QIAN_LI_SYMBOL,
        (Qian, Dui) => QIAN_DUI_SYMBOL,
        (Kun, Qian) => KUN_QIAN_SYMBOL,
        (Kun, Kun) => KUN_KUN_SYMBOL,
        (Kun, Zhen) => KUN_ZHEN_SYMBOL,
        (Kun, Kan) => KUN_KAN_SYMBOL,
        (Kun, Gen) => KUN_GEN_SYMBOL,
        (Kun, Xun) => KUN_XUN_SYMBOL,
        (Kun, Li) => KUN_LI_SYMBOL,
        (Kun, Dui) => KUN_DUI_SYMBOL,
        (Zhen, Qian) => ZHEN_QIAN_SYMBOL,
        (Zhen, Kun) => ZHEN_KUN_SYMBOL,
        (Zhen, Zhen) => ZHEN_ZHEN_SYMBOL,
        (Zhen, Kan) => ZHEN_KAN_SYMBOL,
        (Zhen, Gen) => ZHEN_GEN_SYMBOL,
        (Zhen, Xun) => ZHEN_XUN_SYMBOL,
        (Zhen, Li) => ZHEN_LI_SYMBOL,
        (Zhen, Dui) => ZHEN_DUI_SYMBOL,
        (Kan, Qian) => KAN_QIAN_SYMBOL,
        (Kan, Kun) => KAN_KUN_SYMBOL,
        (Kan, Zhen) => KAN_ZHEN_SYMBOL,
        (Kan, Kan) => KAN_KAN_SYMBOL,
        (Kan, Gen) => KAN_GEN_SYMBOL,
        (Kan, Xun) => KAN_XUN_SYMBOL,
        (Kan, Li) => KAN_LI_SYMBOL,
        (Kan, Dui) => KAN_DUI_SYMBOL,
        (Gen, Qian) => GEN_QIAN_SYMBOL,
        (Gen, Kun) => GEN_KUN_SYMBOL,
        (Gen, Zhen) => GEN_ZHEN_SYMBOL,
        (Gen, Kan) => GEN_KAN_SYMBOL,
        (Gen, Gen) => GEN_GEN_SYMBOL,
        (Gen, Xun) => GEN_XUN_SYMBOL,
        (Gen, Li) => GEN_LI_SYMBOL,
        (Gen, Dui) => GEN_DUI_SYMBOL,
        (Xun, Qian) => XUN_QIAN_SYMBOL,
        (Xun, Kun) => XUN_KUN_SYMBOL,
        (Xun, Zhen) => XUN_ZHEN_SYMBOL,
        (Xun, Kan) => XUN_KAN_SYMBOL,
        (Xun, Gen) => XUN_GEN_SYMBOL,
        (Xun, Xun) => XUN_XUN_SYMBOL,
        (Xun, Li) => XUN_LI_SYMBOL,
        (Xun, Dui) => XUN_DUI_SYMBOL,
        (Li, Qian) => LI_QIAN_SYMBOL,
        (Li, Kun) => LI_KUN_SYMBOL,
        (Li, Zhen) => LI_ZHEN_SYMBOL,
        (Li, Kan) => LI_KAN_SYMBOL,
        (Li, Gen) => LI_GEN_SYMBOL,
        (Li, Xun) => LI_XUN_SYMBOL,
        (Li, Li) => LI_LI_SYMBOL,
        (Li, Dui) => LI_DUI_SYMBOL,
        (Dui, Qian) => DUI_QIAN_SYMBOL,
        (Dui, Kun) => DUI_KUN_SYMBOL,
        (Dui, Zhen) => DUI_ZHEN_SYMBOL,
        (Dui, Kan) => DUI_KAN_SYMBOL,
        (Dui, Gen) => DUI_GEN_SYMBOL,
        (Dui, Xun) => DUI_XUN_SYMBOL,
        (Dui, Li) => DUI_LI_SYMBOL,
        (Dui, Dui) => DUI_DUI_SYMBOL,
    }
}

pub fn king_wen_sequence_number(trigram_name_pair: &TrigramNamePair) -> usize {
    use self::TrigramName::*;
    match trigram_name_pair {
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

pub fn binary_sequence_number(_trigram_name_pair: &TrigramNamePair) -> usize { unimplemented!() }

pub fn mawangdui_sequence_number(_trigram_name_pair: &TrigramNamePair) -> usize { unimplemented!() }

pub fn eight_palaces_sequence_number(_trigram_name_pair: &TrigramNamePair) -> usize { unimplemented!() }


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
