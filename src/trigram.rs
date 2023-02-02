use crate::{
    divination_method::DivinationMethod,
    line::{Line, Line::*},
    symbols::trigram::*,
};
use num_bigint::BigInt;
use serde_derive::Deserialize;
use std::{fmt::Display, fmt::Formatter, io::Write};
use termcolor::{Color, ColorSpec, WriteColor};

const QIAN: Trigram = Trigram(Line::unbroken(), Line::unbroken(), Line::unbroken()); // "☰",
const KUN: Trigram = Trigram(Line::broken(), Line::broken(), Line::broken()); // "☷",
const ZHEN: Trigram = Trigram(Line::broken(), Line::broken(), Line::unbroken()); // "☳",
const KAN: Trigram = Trigram(Line::broken(), Line::unbroken(), Line::broken()); // "☵",
const GEN: Trigram = Trigram(Line::unbroken(), Line::broken(), Line::broken()); // "☶",
const XUN: Trigram = Trigram(Line::unbroken(), Line::unbroken(), Line::broken()); // "☴",
const LI: Trigram = Trigram(Line::unbroken(), Line::broken(), Line::unbroken()); // "☲",
const DUI: Trigram = Trigram(Line::broken(), Line::unbroken(), Line::unbroken()); // "☱",

/// A `Trigram` is a tuple of three [`Line`]s. This is the building
/// block of [`Hexagram`](crate::hexagram::Hexagram)s.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Trigram(pub Line, pub Line, pub Line);

impl Trigram {
    /// Create a new `Hexagram` from random [`Trigram`]s.
    pub fn new_random(divination_method: DivinationMethod) -> Self {
        Trigram(
            Line::new_random(divination_method),
            Line::new_random(divination_method),
            Line::new_random(divination_method),
        )
    }

    /// Get this `Trigram`'s `Line`s. The lines are cloned.
    pub fn lines(&self) -> impl Iterator<Item = &Line> {
        [&self.0, &self.1, &self.2].into_iter()
    }

    /// Print the `Trigram` as large ASCII-art lines.
    pub fn print_big(&self) {
        use crate::symbols::big_line::LINE_SPACER;

        self.0.print_big();
        print!("{LINE_SPACER}");
        self.1.print_big();
        print!("{LINE_SPACER}");
        self.2.print_big();
        print!("{LINE_SPACER}");
    }

    /// Generate a new `Trigram` by using the coin toss method.
    pub fn from_coin_tosses() -> Self {
        Trigram(
            Line::from_coin_tosses(),
            Line::from_coin_tosses(),
            Line::from_coin_tosses(),
        )
    }

    /// Generate a new `Trigram` by using the yarrow stalk method.
    pub fn from_yarrow_stalks() -> Self {
        Trigram(
            Line::from_yarrow_stalks(),
            Line::from_yarrow_stalks(),
            Line::from_yarrow_stalks(),
        )
    }

    /// Get the unicode trigram symbol corresponding to this `Trigram`.
    pub fn symbol(&self) -> &str {
        use TrigramName::*;
        match self.into() {
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

    /// Get the pinyin version of the `Trigram`.
    pub fn pinyin(&self) -> String {
        use TrigramName::*;
        match self.into() {
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

    /// Get the Chinese character version of the `Trigram`.
    pub fn chinese(&self) -> String {
        use TrigramName::*;
        match self.into() {
            Qian => String::from("乾"),
            Kun => String::from("坤"),
            Zhen => String::from("震"),
            Kan => String::from("坎"),
            Gen => String::from("艮"),
            Xun => String::from("巽"),
            Li => String::from("離"),
            Dui => String::from("兌"),
        }
    }

    /// Get the English translation of the `Trigram`.
    pub fn english(&self) -> String {
        use TrigramName::*;
        match self.into() {
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

    /// Get the attribute that the `Trigram` represents.
    pub fn attribute(&self) -> String {
        use TrigramName::*;
        match self.into() {
            Qian => String::from("strong, persisting"),
            Kun => String::from("devoted, yielding"),
            Zhen => String::from("inciting movement"),
            Kan => String::from("dangerous"),
            Gen => String::from("resting, stand-still"),
            Xun => String::from("penetrating"),
            Li => String::from("light-giving"),
            Dui => String::from("pleasure"),
        }
    }

    /// Get the image that the `Trigram` represents.
    pub fn image(&self) -> String {
        use TrigramName::*;
        match self.into() {
            Qian => String::from("heaven, sky"),
            Kun => String::from("earth, ground"),
            Zhen => String::from("thunder"),
            Kan => String::from("water"),
            Gen => String::from("mountain"),
            Xun => String::from("wind, air"),
            Li => String::from("fire, glow"),
            Dui => String::from("lake, marsh"),
        }
    }

    /// Get the family relationship that the `Trigram` represents.
    pub fn family_relationship(&self) -> String {
        use TrigramName::*;
        match self.into() {
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

    /// Get a `Trigram`'s rank/index in the Bagua.
    pub fn number(&self) -> usize {
        use TrigramName::*;
        match self.into() {
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

    pub fn write_to<T>(&self, output: &mut T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: WriteColor + Write,
    {
        // Write out header
        output
            .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(196))))
            .expect("output stream color can be set");

        writeln!(output, "Trigram No. {}  {}", self.number(), self.symbol())?;
        output.reset().expect("output stream color can be reset");

        writeln!(output, "\t{}", self.english())?;
        writeln!(output, "\t{} ({})", self.chinese(), self.pinyin())?;
        writeln!(output)?;

        // Write out attribute
        output
            .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(160))))
            .expect("output stream color can be set");
        writeln!(output, "Attribute:")?;
        output.reset().expect("output stream color can be reset");
        writeln!(output, "\t{}", self.attribute())?;

        // Write out image
        output
            .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(124))))
            .expect("output stream color can be set");
        writeln!(output, "Image in nature:")?;
        output.reset().expect("output stream color can be reset");
        writeln!(output, "\t{}", self.image())?;

        // Write out family relationship
        output
            .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(88))))
            .expect("output stream color can be set");
        writeln!(output, "Family Relationship:")?;
        output.reset().expect("output stream color can be reset");
        writeln!(output, "\t{}", self.family_relationship())?;

        Ok(())
    }

    fn try_from<N>(n: N) -> Result<Self, Error>
    where
        N: Into<BigInt> + TryInto<u8> + Copy,
    {
        let n = TryInto::<u8>::try_into(n).map_err(|_err| Error::IntegerOutOfRange(n.into()))?;

        match n {
            1 => Ok(QIAN),
            2 => Ok(DUI),
            3 => Ok(LI),
            4 => Ok(ZHEN),
            5 => Ok(XUN),
            6 => Ok(KAN),
            7 => Ok(GEN),
            8 => Ok(KUN),
            _ => Err(Error::IntegerOutOfRange(n.into())),
        }
    }
}

impl TryFrom<usize> for Trigram {
    type Error = Error;

    fn try_from(n: usize) -> Result<Self, Self::Error> {
        Trigram::try_from(n)
    }
}

impl TryFrom<u8> for Trigram {
    type Error = Error;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        Trigram::try_from(n)
    }
}

// impl TryFrom<(u32, u32, u32)> for Trigram {
//     type Error = Error;

//     fn try_from((top, middle, bottom): (u32, u32, u32)) -> Result<Self, Self::Error> {
//         Ok(Trigram(
//             Line::try_from(top)?,
//             Line::try_from(middle)?,
//             Line::try_from(bottom)?,
//         ))
//     }
// }

impl AsRef<Trigram> for Trigram {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Named trigrams according to the [Bagua](https://en.wikipedia.org/wiki/Bagua).
#[derive(Deserialize, Clone, Debug)]
pub enum TrigramName {
    Dui,  // Lake or Marsh, 兌, Duì
    Gen,  // Mountain, 艮, Gèn
    Kan,  // Water, 坎, Kǎn
    Kun,  // Earth, 坤, Kūn
    Li,   // Flame or Fire, 離, Lí
    Qian, // Heaven, 乾, Qián
    Xun,  // Wind, 巽, Xùn
    Zhen, // Thunder, 震, Zhèn
}

impl<T> From<T> for TrigramName
where
    T: AsRef<Trigram>,
{
    fn from(trigram: T) -> Self {
        match trigram.as_ref() {
            Trigram(Unbroken { .. }, Unbroken { .. }, Unbroken { .. }) => TrigramName::Qian,
            Trigram(Broken { .. }, Broken { .. }, Broken { .. }) => TrigramName::Kun,
            Trigram(Broken { .. }, Broken { .. }, Unbroken { .. }) => TrigramName::Zhen,
            Trigram(Broken { .. }, Unbroken { .. }, Broken { .. }) => TrigramName::Kan,
            Trigram(Unbroken { .. }, Broken { .. }, Broken { .. }) => TrigramName::Gen,
            Trigram(Unbroken { .. }, Unbroken { .. }, Broken { .. }) => TrigramName::Xun,
            Trigram(Unbroken { .. }, Broken { .. }, Unbroken { .. }) => TrigramName::Li,
            Trigram(Broken { .. }, Unbroken { .. }, Unbroken { .. }) => TrigramName::Dui,
        }
    }
}

impl Display for Trigram {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "{} (No. {})", self.symbol(), self.number())?;
        writeln!(f, "{} - {}", self.pinyin(), self.english())?;
        writeln!(f)?;
        writeln!(f, "Attribute: {}", self.attribute())?;
        writeln!(f, "Image in nature: {}", self.image())?;
        writeln!(f, "Family Relationship: {}", self.family_relationship())?;

        Ok(())
    }
}

/// Errors related to `Trigram`s.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Thrown when creating a `TrigramName` from an integer, if the given integer is not between 1-8 inclusive.
    #[error(
        "Invalid conversion from integer to Trigram. Integer must be between 1-8 inclusive but was {0}"
    )]
    IntegerOutOfRange(BigInt),
    #[error("Failed to create Trigram line: {0}")]
    Line(#[from] crate::line::Error),
}
