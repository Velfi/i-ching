use rand::{
    distributions::{
        Distribution,
        Standard,
    },
    Rng,
};
use std::fmt::{
    Display,
    Error,
    Formatter,
};

/// `Line` represents an individual line within a trigram or hexagram. Hexagrams and trigrams can
/// 'change' into other hexagrams and trigrams based on which lines are marked as 'changing'.
#[derive(Debug)]
pub enum Line {
    BrokenChanging,
    Broken,
    Unbroken,
    UnbrokenChanging,
}

impl Line {
    /// `settle` a line with or without 'changes'. Used to fetch a 'before' and after
    /// 'hexagram' with which to make a prediction of how things will change between
    /// now and the future.
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

    pub fn print_big(&self) {
        use crate::symbols::big_line::*;
        match self {
            Line::BrokenChanging => print!("{}", BIG_BROKEN_CHANGING),
            Line::Broken => print!("{}", BIG_BROKEN),
            Line::Unbroken => print!("{}", BIG_UNBROKEN),
            Line::UnbrokenChanging => print!("{}", BIG_UNBROKEN_CHANGING),
        };
    }
}

impl Default for Line {
    fn default() -> Self {
        rand::random()
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let line_string = match self {
            Line::BrokenChanging => "-X-",
            Line::Broken => "- -",
            Line::Unbroken => "---",
            Line::UnbrokenChanging => "-O-",
        };
        write!(f, "{}", line_string)
    }
}

impl Distribution<Line> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Line {
        match rng.gen_range(6, 10) {
            6 => Line::BrokenChanging,
            7 => Line::Broken,
            8 => Line::Unbroken,
            _ => Line::UnbrokenChanging,
        }
    }
}

