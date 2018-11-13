use std::{
    error::Error,
    fmt::Display,
    fmt::Formatter,
};

use serde_derive::Deserialize;

use crate::{
    line::Line,
    symbols::{
        hexagram::*,
        trigram::*,
    }
};

/// A `Trigram` is a tuple of three [`Line`](../line/enum.Line.html)s. It can be converted into
/// a [`TrigramName`](enum.TrigramName.html) by calling `get_name()` on it. This is the building
/// block of a [`Hexagram`](../hexagram/struct.Hexagram.html).
#[derive(Clone, Debug)]
pub struct Trigram(pub Line, pub Line, pub Line);

impl Trigram {
    /// Get a `TrigramName` from a `Trigram` with or without changes.
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
            // Per the "settling" above, this is unreachable
            _ => unreachable!()
        }
    }

    /// Get this `Trigram`'s `Line`s. The lines are cloned.
    pub fn get_lines_as_vec(&self) -> Vec<Line> {
        vec![
            self.0.clone(),
            self.1.clone(),
            self.2.clone(),
        ]
    }

    /// Print the `Trigram` as large ASCII-art lines.
    pub fn print_big(&self) {
        use crate::symbols::big_line::BIG_LINE_SPACER;

        self.0.print_big();
        print!("{}", BIG_LINE_SPACER);
        self.1.print_big();
        print!("{}", BIG_LINE_SPACER);
        self.2.print_big();
        print!("{}", BIG_LINE_SPACER);
    }

    // Generate a new `Trigram` by using the coin toss method.
    pub fn from_coin_tosses() -> Self {
        Trigram(
            Line::from_coin_tosses(),
            Line::from_coin_tosses(),
            Line::from_coin_tosses(),
        )
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

/// Named trigrams according to the [Bagua](https://en.wikipedia.org/wiki/Bagua). This represents a
/// "settled" `Trigram` that will not "change."
#[derive(Deserialize, Clone, Debug)]
pub enum TrigramName {
    Dui,
    // 兌, Duì
    Gen,
    // 艮, Gèn
    Kan,
    // 坎, Kǎn
    Kun,
    // 坤, Kūn
    Li,
    // 離, Lí
    Qian,
    // 乾, Qián
    Xun,
    // 巽, Xùn
    Zhen, // 震, Zhèn
}

impl TrigramName {
    /// Get the unicode trigram symbol corresponding to this `TrigramName`.
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

    /// Get the romanized pronunciation of the `TrigramName`.
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

    /// Get the English translation of the `TrigramName`.
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

    /// Get the attribute that the `TrigramName` represents.
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

    /// Get the image that the `TrigramName` represents.
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

    /// Get the family relationship that the `TrigramName` represents.
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

    /// Create a new `TrigramName` from the trigram's rank/index.
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

    /// Get a `TrigramName`'s rank/index in the Bagua.
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

/// A tuple containing two [`TrigramName`](enum.TrigramName.html)s. In the future, functionality
/// provided by this struct will be rolled in `Hexagram`s.
#[derive(Clone, Debug)]
pub struct TrigramNamePair(pub TrigramName, pub TrigramName);

impl TrigramNamePair {
    /// Create a new `TrigramNamePair` from two `TrigramName`s.
    pub fn new(above: TrigramName, below: TrigramName) -> Self {
        TrigramNamePair(above, below)
    }

    /// Get a `&str` to a unicode symbol representing the `TrigramNamePair`.
    pub fn as_symbol(&self) -> &'static str {
        use self::TrigramName::*;
        match self {
            TrigramNamePair(Qian, Qian) => QIAN_QIAN_SYMBOL,
            TrigramNamePair(Qian, Kun) => QIAN_KUN_SYMBOL,
            TrigramNamePair(Qian, Zhen) => QIAN_ZHEN_SYMBOL,
            TrigramNamePair(Qian, Kan) => QIAN_KAN_SYMBOL,
            TrigramNamePair(Qian, Gen) => QIAN_GEN_SYMBOL,
            TrigramNamePair(Qian, Xun) => QIAN_XUN_SYMBOL,
            TrigramNamePair(Qian, Li) => QIAN_LI_SYMBOL,
            TrigramNamePair(Qian, Dui) => QIAN_DUI_SYMBOL,
            TrigramNamePair(Kun, Qian) => KUN_QIAN_SYMBOL,
            TrigramNamePair(Kun, Kun) => KUN_KUN_SYMBOL,
            TrigramNamePair(Kun, Zhen) => KUN_ZHEN_SYMBOL,
            TrigramNamePair(Kun, Kan) => KUN_KAN_SYMBOL,
            TrigramNamePair(Kun, Gen) => KUN_GEN_SYMBOL,
            TrigramNamePair(Kun, Xun) => KUN_XUN_SYMBOL,
            TrigramNamePair(Kun, Li) => KUN_LI_SYMBOL,
            TrigramNamePair(Kun, Dui) => KUN_DUI_SYMBOL,
            TrigramNamePair(Zhen, Qian) => ZHEN_QIAN_SYMBOL,
            TrigramNamePair(Zhen, Kun) => ZHEN_KUN_SYMBOL,
            TrigramNamePair(Zhen, Zhen) => ZHEN_ZHEN_SYMBOL,
            TrigramNamePair(Zhen, Kan) => ZHEN_KAN_SYMBOL,
            TrigramNamePair(Zhen, Gen) => ZHEN_GEN_SYMBOL,
            TrigramNamePair(Zhen, Xun) => ZHEN_XUN_SYMBOL,
            TrigramNamePair(Zhen, Li) => ZHEN_LI_SYMBOL,
            TrigramNamePair(Zhen, Dui) => ZHEN_DUI_SYMBOL,
            TrigramNamePair(Kan, Qian) => KAN_QIAN_SYMBOL,
            TrigramNamePair(Kan, Kun) => KAN_KUN_SYMBOL,
            TrigramNamePair(Kan, Zhen) => KAN_ZHEN_SYMBOL,
            TrigramNamePair(Kan, Kan) => KAN_KAN_SYMBOL,
            TrigramNamePair(Kan, Gen) => KAN_GEN_SYMBOL,
            TrigramNamePair(Kan, Xun) => KAN_XUN_SYMBOL,
            TrigramNamePair(Kan, Li) => KAN_LI_SYMBOL,
            TrigramNamePair(Kan, Dui) => KAN_DUI_SYMBOL,
            TrigramNamePair(Gen, Qian) => GEN_QIAN_SYMBOL,
            TrigramNamePair(Gen, Kun) => GEN_KUN_SYMBOL,
            TrigramNamePair(Gen, Zhen) => GEN_ZHEN_SYMBOL,
            TrigramNamePair(Gen, Kan) => GEN_KAN_SYMBOL,
            TrigramNamePair(Gen, Gen) => GEN_GEN_SYMBOL,
            TrigramNamePair(Gen, Xun) => GEN_XUN_SYMBOL,
            TrigramNamePair(Gen, Li) => GEN_LI_SYMBOL,
            TrigramNamePair(Gen, Dui) => GEN_DUI_SYMBOL,
            TrigramNamePair(Xun, Qian) => XUN_QIAN_SYMBOL,
            TrigramNamePair(Xun, Kun) => XUN_KUN_SYMBOL,
            TrigramNamePair(Xun, Zhen) => XUN_ZHEN_SYMBOL,
            TrigramNamePair(Xun, Kan) => XUN_KAN_SYMBOL,
            TrigramNamePair(Xun, Gen) => XUN_GEN_SYMBOL,
            TrigramNamePair(Xun, Xun) => XUN_XUN_SYMBOL,
            TrigramNamePair(Xun, Li) => XUN_LI_SYMBOL,
            TrigramNamePair(Xun, Dui) => XUN_DUI_SYMBOL,
            TrigramNamePair(Li, Qian) => LI_QIAN_SYMBOL,
            TrigramNamePair(Li, Kun) => LI_KUN_SYMBOL,
            TrigramNamePair(Li, Zhen) => LI_ZHEN_SYMBOL,
            TrigramNamePair(Li, Kan) => LI_KAN_SYMBOL,
            TrigramNamePair(Li, Gen) => LI_GEN_SYMBOL,
            TrigramNamePair(Li, Xun) => LI_XUN_SYMBOL,
            TrigramNamePair(Li, Li) => LI_LI_SYMBOL,
            TrigramNamePair(Li, Dui) => LI_DUI_SYMBOL,
            TrigramNamePair(Dui, Qian) => DUI_QIAN_SYMBOL,
            TrigramNamePair(Dui, Kun) => DUI_KUN_SYMBOL,
            TrigramNamePair(Dui, Zhen) => DUI_ZHEN_SYMBOL,
            TrigramNamePair(Dui, Kan) => DUI_KAN_SYMBOL,
            TrigramNamePair(Dui, Gen) => DUI_GEN_SYMBOL,
            TrigramNamePair(Dui, Xun) => DUI_XUN_SYMBOL,
            TrigramNamePair(Dui, Li) => DUI_LI_SYMBOL,
            TrigramNamePair(Dui, Dui) => DUI_DUI_SYMBOL,
        }
    }

    /// Get the number of the `TrigramNamePair` according to the King Wen Sequence.
    pub fn king_wen_sequence_number(&self) -> usize {
        use self::TrigramName::*;
        match self {
            TrigramNamePair(Qian, Qian) => 1,
            TrigramNamePair(Qian, Kun) => 12,
            TrigramNamePair(Qian, Zhen) => 25,
            TrigramNamePair(Qian, Kan) => 6,
            TrigramNamePair(Qian, Gen) => 33,
            TrigramNamePair(Qian, Xun) => 44,
            TrigramNamePair(Qian, Li) => 13,
            TrigramNamePair(Qian, Dui) => 10,
            TrigramNamePair(Kun, Qian) => 11,
            TrigramNamePair(Kun, Kun) => 2,
            TrigramNamePair(Kun, Zhen) => 24,
            TrigramNamePair(Kun, Kan) => 7,
            TrigramNamePair(Kun, Gen) => 15,
            TrigramNamePair(Kun, Xun) => 46,
            TrigramNamePair(Kun, Li) => 36,
            TrigramNamePair(Kun, Dui) => 19,
            TrigramNamePair(Zhen, Qian) => 34,
            TrigramNamePair(Zhen, Kun) => 16,
            TrigramNamePair(Zhen, Zhen) => 51,
            TrigramNamePair(Zhen, Kan) => 40,
            TrigramNamePair(Zhen, Gen) => 62,
            TrigramNamePair(Zhen, Xun) => 32,
            TrigramNamePair(Zhen, Li) => 55,
            TrigramNamePair(Zhen, Dui) => 54,
            TrigramNamePair(Kan, Qian) => 5,
            TrigramNamePair(Kan, Kun) => 8,
            TrigramNamePair(Kan, Zhen) => 3,
            TrigramNamePair(Kan, Kan) => 29,
            TrigramNamePair(Kan, Gen) => 39,
            TrigramNamePair(Kan, Xun) => 48,
            TrigramNamePair(Kan, Li) => 63,
            TrigramNamePair(Kan, Dui) => 60,
            TrigramNamePair(Gen, Qian) => 26,
            TrigramNamePair(Gen, Kun) => 23,
            TrigramNamePair(Gen, Zhen) => 27,
            TrigramNamePair(Gen, Kan) => 4,
            TrigramNamePair(Gen, Gen) => 52,
            TrigramNamePair(Gen, Xun) => 18,
            TrigramNamePair(Gen, Li) => 22,
            TrigramNamePair(Gen, Dui) => 41,
            TrigramNamePair(Xun, Qian) => 9,
            TrigramNamePair(Xun, Kun) => 20,
            TrigramNamePair(Xun, Zhen) => 42,
            TrigramNamePair(Xun, Kan) => 59,
            TrigramNamePair(Xun, Gen) => 53,
            TrigramNamePair(Xun, Xun) => 57,
            TrigramNamePair(Xun, Li) => 37,
            TrigramNamePair(Xun, Dui) => 61,
            TrigramNamePair(Li, Qian) => 14,
            TrigramNamePair(Li, Kun) => 35,
            TrigramNamePair(Li, Zhen) => 21,
            TrigramNamePair(Li, Kan) => 64,
            TrigramNamePair(Li, Gen) => 56,
            TrigramNamePair(Li, Xun) => 50,
            TrigramNamePair(Li, Li) => 30,
            TrigramNamePair(Li, Dui) => 38,
            TrigramNamePair(Dui, Qian) => 43,
            TrigramNamePair(Dui, Kun) => 45,
            TrigramNamePair(Dui, Zhen) => 17,
            TrigramNamePair(Dui, Kan) => 47,
            TrigramNamePair(Dui, Gen) => 31,
            TrigramNamePair(Dui, Xun) => 28,
            TrigramNamePair(Dui, Li) => 49,
            TrigramNamePair(Dui, Dui) => 58,
        }
    }

    /// Get the number of the `TrigramNamePair` according to the [Binary Sequence](https://en.wikipedia.org/wiki/Bagua#Fu_Xi_%22Earlier_Heaven%22).
    /// **currently unimplemented!**
    pub fn binary_sequence_number(&self) -> usize { unimplemented!() }

    /// Get the number of the `TrigramNamePair` according to the [Mawangdui Sequence](https://en.wikipedia.org/wiki/Mawangdui).
    /// **currently unimplemented!**
    pub fn mawangdui_sequence_number(&self) -> usize { unimplemented!() }
}

/// Errors related to `Trigram`s.
#[derive(Debug)]
pub enum TrigramError {
    /// Thrown when creating a `TrigramName` from an integer, if the given integer is not between 1-8 inclusive.
    IntegerOutOfRange,
}

static INTEGER_OUT_OF_RANGE_ERROR_MSG: &str =
    "Invalid conversion from usize to TrigramName, make sure your usize is between 1-8 inclusive";

impl Display for TrigramError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        use self::TrigramError::*;
        match self {
            IntegerOutOfRange => write!(f, "{}", INTEGER_OUT_OF_RANGE_ERROR_MSG),
        }
    }
}

impl Error for TrigramError {
    fn description(&self) -> &str {
        use self::TrigramError::*;
        match self {
            IntegerOutOfRange => INTEGER_OUT_OF_RANGE_ERROR_MSG,
        }
    }
}
