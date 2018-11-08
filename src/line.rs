use std::fmt::{
    Display,
    Error,
    Formatter,
};

use rand::{
    distributions::{
        Distribution,
        Standard,
    },
    Rng,
};

/// `Line` represents an individual line within a trigram or hexagram. Hexagrams and trigrams can
/// "change" into other hexagrams and trigrams based on which lines are marked as "changing".
#[derive(Debug)]
pub enum Line {
    BrokenChanging,
    Broken,
    Unbroken,
    UnbrokenChanging,
}

impl Line {
    /// `settle` a line with or without "changes". Used during divination to get a pre- and
    /// post-changes hexagram. The "primary" hexagram's meaning can be changed by the "relating"
    /// hexagram.
    pub fn settle(&self, with_change: bool) -> Line {
        use self::Line::*;
        if with_change {
            match self {
                Broken | UnbrokenChanging => Broken,
                Unbroken | BrokenChanging => Unbroken,
            }
        } else {
            match self {
                Broken | BrokenChanging => Broken,
                Unbroken | UnbrokenChanging => Unbroken,
            }
        }
    }

    /// Print the `Line` as large ASCII art.
    pub fn print_big(&self) {
        use crate::symbols::big_line::*;
        use self::Line::*;
        match self {
            BrokenChanging => print!("{}", BIG_BROKEN_CHANGING),
            Broken => print!("{}", BIG_BROKEN),
            Unbroken => print!("{}", BIG_UNBROKEN),
            UnbrokenChanging => print!("{}", BIG_UNBROKEN_CHANGING),
        };
    }
}

impl Default for Line {
    /// Generate a random `Line` that may or may not be "changing".
    fn default() -> Self {
        rand::random()
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use self::Line::*;
        let line_string = match self {
            BrokenChanging => "-X-",
            Broken => "- -",
            Unbroken => "---",
            UnbrokenChanging => "-O-",
        };
        write!(f, "{}", line_string)
    }
}

impl Distribution<Line> for Standard {
    /// Generate a random `Line` that may or may not be "changing".
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Line {
        match rng.gen_range(6, 10) {
            6 => Line::BrokenChanging,
            7 => Line::Broken,
            8 => Line::Unbroken,
            _ => Line::UnbrokenChanging,
        }
    }
}

