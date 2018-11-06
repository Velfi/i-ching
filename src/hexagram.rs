use crate::symbols::big_line::BIG_LINE_SPACER;
use crate::trigram::{
    self,
    Trigram,
    trigram_name_pair_as_symbol,
    TrigramNamePair,
};

#[derive(Clone, Copy)]
pub enum HexagramOrdering {
    KingWen,
    // a.k.a Fu Xi sequence, , Shao Yong sequence
    Binary,
    Mawangdui,
    EightPalaces,
}

pub struct Hexagram {
    _above: Trigram,
    _below: Trigram,
}

impl Hexagram {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn symbol(&self, with_changes: bool) -> &str {
        trigram_name_pair_as_symbol(&self.as_trigram_name_pair(with_changes))
    }

    pub fn as_number(&self, with_changes: bool, sequence: HexagramOrdering) -> usize {
        use self::HexagramOrdering::*;
        let trigram_name_pair: TrigramNamePair = self.as_trigram_name_pair(with_changes);

        match sequence {
            KingWen => trigram::king_wen_sequence_number(&trigram_name_pair),
            Binary => trigram::binary_sequence_number(&trigram_name_pair),
            Mawangdui => trigram::mawangdui_sequence_number(&trigram_name_pair),
            EightPalaces => trigram::eight_palaces_sequence_number(&trigram_name_pair),
        }
    }

    pub fn as_trigram_name_pair(&self, with_changes: bool) -> TrigramNamePair {
        (self._above.get_name(with_changes), self._below.get_name(with_changes))
    }

    pub fn print_big(&self) {
        print!("{}", BIG_LINE_SPACER);
        self._above.print_big();
        self._below.print_big();
    }
}

impl Default for Hexagram {
    fn default() -> Self {
        Hexagram {
            _above: Trigram::default(),
            _below: Trigram::default(),
        }
    }
}
