use crate::{
    divination_method::DivinationMethod, line::Line, symbols::big_line::LINE_SPACER,
    trigram::Trigram,
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
    above: Trigram,
    below: Trigram,
}

impl Hexagram {
    /// Create a new `Hexagram` from two [`Trigram`]s. The `Trigram`s are consumed in the process.
    pub fn new(above: Trigram, below: Trigram) -> Self {
        Hexagram { above, below }
    }

    pub fn above(&self) -> &Trigram {
        &self.above
    }

    pub fn below(&self) -> &Trigram {
        &self.below
    }

    /// Create a new `Hexagram` from random [`Trigram`]s.
    pub fn new_random(divination_method: DivinationMethod) -> Self {
        Hexagram {
            above: Trigram::new_random(divination_method),
            below: Trigram::new_random(divination_method),
        }
    }

    /// Get the unicode symbol representing this Hexagram. The symbol is retrieved from the given
    /// `HexagramRepository`.
    pub fn symbol(&self, with_changes: bool) -> &str {
        use crate::symbols::hexagram::*;
        use crate::trigram::TrigramName::*;

        let relating_hexagram = with_changes.then(|| self.relating_hexagram()).flatten();
        let hexagram = relating_hexagram.as_ref().unwrap_or(self);

        match (hexagram.above.into(), hexagram.below.into()) {
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

    /// Get a `Vec` of `usize`s representing the positions of lines that are marked as "changing".
    pub fn get_changing_line_positions(&self) -> Vec<usize> {
        self.lines()
            .enumerate()
            .filter_map(|(index, &line)| {
                if matches!(line, Line::Broken { changing: true })
                    || matches!(line, Line::Unbroken { changing: true })
                {
                    Some(index + 1)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get a vec of this `Hexagram`'s `Line`s _("above" lines followed by "below" lines.)_
    pub fn lines(&self) -> impl Iterator<Item = &Line> {
        self.above.lines().chain(self.below.lines())
    }

    /// Print the `Hexagram` as large ASCII-art lines.
    pub fn print_big(&self) {
        print!("{LINE_SPACER}");
        self.above.print_big();
        self.below.print_big();
    }

    /// Return this Hexagram's "relating" Hexagram, if it has one. Only Hexagrams with changing lines
    /// have relating Hexagrams.
    pub fn relating_hexagram(&self) -> Option<Self> {
        let Hexagram { above, below } = self;
        let Trigram(a1, a2, a3) = above;
        let Trigram(b1, b2, b3) = below;
        let lines_are_changing = [a1, a2, a3, b1, b2, b3]
            .iter()
            .any(|line| line.is_changing());

        if lines_are_changing {
            Some(Hexagram::new(
                Trigram(a1.settle(), a2.settle(), a3.settle()),
                Trigram(b1.settle(), b2.settle(), b3.settle()),
            ))
        } else {
            None
        }
    }
}
