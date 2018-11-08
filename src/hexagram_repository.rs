use std::{
    error::Error,
    fmt::{
        self,
        Display,
        Formatter,
    },
};

use serde_derive::Deserialize;

use crate::hexagram::HexagramOrdering;

/// A generic interface for some repository of information on individual hexagrams. For example,
/// this repository could be a local JSON file or even some data in a remote server. It allows
/// one to fetch data for an individual hexagram by way of returning a generic `HexagramInfo`
/// object.
///
/// Repositories must be "initialized" before being used.
pub trait HexagramRepository {
    /// Fetch an `Option` possibly containing a ref to a `HexagramInfo` object. The ordering of
    /// objects in the repository should conform to a known sequence, which can be checked
    /// by calling `get_ordering`.
    /// [`KingWen` Sequence](../hexagram/enum.HexagramOrdering.html) is typical.
    fn get_by_number(&self, number: usize) -> Option<&dyn HexagramInfo>;
    /// Check to see if this repository has been initialized. If `false`, the repository may fail
    /// to function correctly or even panic if other methods are called.
    fn get_is_initialized(&self) -> bool;
    /// Get the ordering sequence of this repository.
    /// [`KingWen` Sequence](../hexagram/enum.HexagramOrdering.html) is typical.
    fn get_ordering(&self) -> &HexagramOrdering;
    /// Do anything necessary to initialize this repository. Returns an empty `Result`
    /// if successful and returns a boxed error otherwise.
    fn initialize(&mut self) -> Result<(), Box<Error>>;
    /// Create a new, uninitialized repository.
    fn new() -> Self;
}

/// Associates the meaning of a changing line with the position of a changing line in a hexagram.
#[derive(Deserialize)]
pub struct ChangingLineMeaning {
    pub position: usize,
    pub meaning: String,
}

/// A generic interface to some data for a hexagram. `HexagramInfo` objects are provided by
/// a `HexagramRepository`, and are referred to by number.
pub trait HexagramInfo {
    /// Get the hexagram name as Chinese characters.
    fn get_chinese_name(&self) -> &str;
    /// Get the hexagram name as a localized `&str`.
    fn get_localized_name(&self) -> &str;
    /// Get info on the "images" associated with the hexagram.
    fn get_images(&self) -> &str;
    /// Get info on a judgement associated with the hexagram.
    fn get_judgement(&self) -> &str;
    /// For a list of hexagram lines that are changing, get information on how those changing lines
    /// affect the meaning of the hexagram *(if they do at all.)*
    fn get_line_meanings(&self, changing_lines: &[usize]) -> Vec<&ChangingLineMeaning>;
    /// Get the number of the hexagram. The number of a hexagram changes for different "sequences".
    /// This function should return the appropriate number given the "sequence" of its parent
    /// repository. Repositories have a fixed ordering.
    fn get_number(&self) -> usize;
    /// Get a `&str` to a unicode symbol representing the Hexagram.
    fn get_symbol(&self) -> &str;
}

impl Display for &dyn HexagramInfo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "  {} (No. {})", self.get_symbol(), self.get_number());
        writeln!(f, "  {} - {}", self.get_chinese_name(), self.get_localized_name());
        writeln!(f);
        writeln!(f, "  Judgement:");
        writeln!(f, "{}", self.get_judgement());
        writeln!(f, "  Images:");
        writeln!(f, "{}", self.get_images());

        Ok(())
    }
}