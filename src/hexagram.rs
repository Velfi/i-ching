use crate::{
    line::Line,
    symbols::big_line::BIG_LINE_SPACER,
    trigram::{
        Trigram,
        TrigramNamePair,
    },
};

/// The 64 Hexagrams have several different orderings, the most
/// common of which is the King Wen sequence.
/// [See here for more details / history](https://en.wikipedia.org/wiki/King_Wen_sequence)
#[derive(Clone, Copy)]
pub enum HexagramOrdering {
    /// The most common sequence
    KingWen,
    /// a.k.a Fu Xi sequence, Shao Yong sequence
    Binary,
    /// From the [Mawangdui Silk Texts](https://en.wikipedia.org/wiki/Mawangdui_Silk_Texts)
    Mawangdui,
}

/// A `Hexagram` is a collection of lines divided into two groups: lines above and lines below.
/// The order of the lines determines the specific hexagram (the primary hexagram) and its
/// meaning. If lines are marked as "changing", then a second hexagram (the relating hexagram)
/// will be produced that provides additional meaning. Special attention should be paid to
/// "changing" lines as they can change the meaning of the primary hexagram.
/// [See here for more details / history](https://en.wikipedia.org/wiki/Hexagram_\(I_Ching\))
pub struct Hexagram {
    _above: Trigram,
    _below: Trigram,
}

impl Hexagram {
    /// Create a new `Hexagram` from two [`Trigram`](../trigram/struct.Trigram.html)s. The `Trigram`s are consumed in the process.
    pub fn new(above: Trigram, below: Trigram) -> Self {
        Hexagram {
            _above: above,
            _below: below,
        }
    }

    /// Create a new `Hexagram` from random [`Trigram`](../trigram/struct.Trigram.html)s. An alias for `default()`.
    pub fn new_random() -> Self {
        Self::default()
    }

    // Generate a new `Hexagram` by using the coin toss method.
    pub fn from_coin_tosses() -> Self {
        Hexagram {
            _above: Trigram::from_coin_tosses(),
            _below: Trigram::from_coin_tosses(),
        }
    }

    /// Get a `&str` to a unicode symbol representing the Hexagram.
    pub fn symbol(&self, with_changes: bool) -> &str {
        &self._as_trigram_name_pair(with_changes).as_symbol()
    }

    /// Get a `Vec` of `usize`s representing the positions of lines that are marked as "changing".
    pub fn get_changing_line_positions(&self) -> Vec<usize> {
        self.get_lines_as_vec()
            .iter()
            .enumerate()
            .filter_map(|enumerated_line|
                match enumerated_line {
                    (index, Line::BrokenChanging) |
                    (index, Line::UnbrokenChanging) => Some(index),
                    _ => None,
                })
            .collect()
    }

    /// Get the number of this `Hexagram` pre- or post-change, according to a given sequence.
    /// If `with_changes` is `true` but a hexagram has no changing lines, the resulting
    /// number will be the same as if `with_changes` was `false`.
    pub fn as_number(&self, with_changes: bool, sequence: HexagramOrdering) -> usize {
        use self::HexagramOrdering::*;
        let trigram_name_pair: TrigramNamePair = self._as_trigram_name_pair(with_changes);

        match sequence {
            KingWen => trigram_name_pair.king_wen_sequence_number(),
            Binary => trigram_name_pair.binary_sequence_number(),
            Mawangdui => trigram_name_pair.mawangdui_sequence_number(),
        }
    }

    /// Get a vec of this `Hexagram`'s `Line`s *("above" lines followed by "below" lines.)*.
    /// The lines are cloned.
    pub fn get_lines_as_vec(&self) -> Vec<Line> {
        let mut resulting_vec = Vec::new();
        resulting_vec.extend_from_slice(&self._above.get_lines_as_vec());
        resulting_vec.extend_from_slice(&self._below.get_lines_as_vec());

        resulting_vec
    }

    /// Print the `Hexagram` as large ASCII-art lines.
    pub fn print_big(&self) {
        print!("{}", BIG_LINE_SPACER);
        self._above.print_big();
        self._below.print_big();
    }

    /// A utility function to get a `TrigramNamePair` pre- or post-changes. Mainly used to
    /// interface with utilities in the `trigram` module.
    fn _as_trigram_name_pair(&self, with_changes: bool) -> TrigramNamePair {
        TrigramNamePair(self._above.get_name(with_changes), self._below.get_name(with_changes))
    }
}

impl Default for Hexagram {
    /// Create a new `Hexagram` from random `Trigram`s.
    fn default() -> Self {
        Hexagram {
            _above: Trigram::default(),
            _below: Trigram::default(),
        }
    }
}
