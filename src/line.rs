use crate::divination_method::DivinationMethod;
use num_bigint::BigInt;
use std::fmt;

/// `Line` represents an individual line within a trigram or hexagram. Hexagrams and trigrams can
/// "change" into other hexagrams and trigrams based on which lines are marked as "changing".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Line {
    Unbroken { changing: bool },
    Broken { changing: bool },
}

impl Line {
    pub fn new_random(divination_method: DivinationMethod) -> Self {
        match divination_method {
            DivinationMethod::CoinToss => Self::from_coin_tosses(),
            DivinationMethod::AncientYarrowStalk => Self::from_yarrow_stalks(),
        }
    }

    pub fn is_changing(&self) -> bool {
        match self {
            Self::Broken { changing } | Self::Unbroken { changing } => *changing,
        }
    }

    /// young yin a.k.a. yin unchanging
    pub const fn broken() -> Self {
        Self::Broken { changing: false }
    }

    /// old yin a.k.a. yin changing into yang
    pub const fn broken_changing() -> Self {
        Self::Broken { changing: true }
    }

    /// young yang a.k.a. yang unchanging
    pub const fn unbroken() -> Self {
        Self::Unbroken { changing: false }
    }

    /// old yang a.k.a. yang changing into yin
    pub const fn unbroken_changing() -> Self {
        Self::Unbroken { changing: true }
    }

    // Generate a new `Line` by using the coin toss method.
    // https://en.wikipedia.org/wiki/I_Ching_divination#Coins
    pub fn from_coin_tosses() -> Self {
        match fastrand::u8(1..=16) {
            // 2/16
            1..=2 => Line::broken_changing(),
            // 6/16
            3..=8 => Line::broken(),
            // 2/16
            9..=10 => Line::unbroken_changing(),
            // 6/16
            11..=16 => Line::unbroken(),
            _ => unreachable!(),
        }
    }

    // Generate a new `Line`s with a random distribution based on the ancient yarrow stalk method.
    // https://en.wikipedia.org/wiki/I_Ching_divination#Yarrow_stalks
    pub fn from_yarrow_stalks() -> Self {
        match fastrand::u8(1..=16) {
            // 1/16
            1 => Line::broken_changing(),
            // 7/16
            2..=8 => Line::broken(),
            // 3/16
            9..=11 => Line::unbroken_changing(),
            // 5/16
            12..=16 => Line::unbroken(),
            _ => unreachable!(),
        }
    }

    /// `settle` a line that might be "changing". If the line is "changing", it will be settled to
    /// its opposite state. If the line is not "changing", it will remain unchanged.
    pub fn settle(&self) -> Line {
        match self {
            Self::Broken { changing: false } | Self::Unbroken { changing: true } => Self::broken(),
            Self::Unbroken { changing: false } | Self::Broken { changing: true } => {
                Self::unbroken()
            }
        }
    }

    /// Print the `Line` as large ASCII art.
    pub fn print_big(&self) {
        use crate::symbols::big_line::*;
        match self {
            Self::Broken { changing: true } => print!("{BROKEN_CHANGING}"),
            Self::Broken { changing: false } => print!("{BROKEN}"),
            Self::Unbroken { changing: true } => print!("{UNBROKEN_CHANGING}"),
            Self::Unbroken { changing: false } => print!("{UNBROKEN}"),
        };
    }

    fn try_from<N>(n: N) -> Result<Self, Error>
    where
        N: Into<BigInt> + TryInto<u8> + Copy,
    {
        let n = TryInto::<u8>::try_into(n).map_err(|_err| Error::IntegerOutOfRange(n.into()))?;

        match n {
            6 => Ok(Line::broken_changing()),
            7 => Ok(Line::unbroken()),
            8 => Ok(Line::broken()),
            9 => Ok(Line::unbroken_changing()),
            _ => Err(Error::IntegerOutOfRange(n.into())),
        }
    }
}

impl TryFrom<u8> for Line {
    type Error = Error;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        Line::try_from(n)
    }
}

impl TryFrom<u32> for Line {
    type Error = Error;

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        Line::try_from(n)
    }
}

impl TryFrom<i32> for Line {
    type Error = Error;

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        Line::try_from(n)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let line_string = match self {
            Line::Broken { changing: true } => "-X-",
            Line::Broken { changing: false } => "- -",
            Line::Unbroken { changing: true } => "-O-",
            Line::Unbroken { changing: false } => "---",
        };
        write!(f, "{line_string}")
    }
}

/// Errors related to `Line`s.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Thrown when creating a `Line` from an integer, if the given integer is not between 6-9 inclusive.
    #[error(
        "Invalid conversion from integer to Line. Integer must be between 6-9 inclusive but was {0}"
    )]
    IntegerOutOfRange(BigInt),
}
