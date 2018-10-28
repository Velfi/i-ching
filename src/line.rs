use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;
use serde_derive::Deserialize;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum Line {
    BrokenChanging,
    Broken,
    Unbroken,
    UnbrokenChanging,
}

impl Line {
    pub fn settle(self, with_change: bool) -> Line {
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

#[derive(Deserialize)]
pub struct LineMeaning {
    position: String,
    meaning: String
}