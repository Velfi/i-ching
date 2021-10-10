use crate::coin::Coin;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt::{Display, Error, Formatter};

/// `Line` represents an individual line within a trigram or hexagram. Hexagrams and trigrams can
/// "change" into other hexagrams and trigrams based on which lines are marked as "changing".
#[derive(Debug, Clone)]
pub enum Line {
    BrokenChanging,
    Unbroken,
    Broken,
    UnbrokenChanging,
}

impl Line {
    pub fn from_usize(n: usize) -> Self {
        match n {
            6 => Line::BrokenChanging,
            7 => Line::Unbroken,
            8 => Line::Broken,
            9 => Line::UnbrokenChanging,
            _ => unreachable!(),
        }
    }
    // Generate a new `Line` by using the coin toss method.
    pub fn from_coin_tosses() -> Self {
        let toss_results: [Coin; 3] = [rand::random(), rand::random(), rand::random()];

        let toss_total = toss_results.iter().fold(0usize, |sum, coin| {
            sum + match coin {
                Coin::Tails => 2,
                Coin::Heads => 3,
            }
        });

        match toss_total {
            6 => Line::BrokenChanging,
            7 => Line::Unbroken,
            8 => Line::Broken,
            9 => Line::UnbrokenChanging,
            _ => unreachable!(),
        }
    }

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
        use self::Line::*;
        use crate::symbols::big_line::*;
        match self {
            BrokenChanging => print!("{}", BIG_BROKEN_CHANGING),
            Unbroken => print!("{}", BIG_UNBROKEN),
            Broken => print!("{}", BIG_BROKEN),
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
            Unbroken => "---",
            Broken => "- -",
            UnbrokenChanging => "-O-",
        };
        write!(f, "{}", line_string)
    }
}

impl Distribution<Line> for Standard {
    /// Generate a random `Line` that may or may not be "changing".
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Line {
        match rng.gen_range(6..10) {
            6 => Line::BrokenChanging,
            7 => Line::Unbroken,
            8 => Line::Broken,
            9 => Line::UnbrokenChanging,
            _ => unreachable!(),
        }
    }
}
