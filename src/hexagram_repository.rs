use crate::{hexagram::HexagramOrdering, Hexagram};
use std::error::Error;

/// A generic interface for some repository of information on individual hexagrams. For example,
/// this repository could be a local JSON file or even some data in a remote server. It allows
/// one to fetch data for an individual hexagram by way of returning a generic `HexagramInfo`
/// object.
///
/// Repositories must be "initialized" before being used.
pub trait HexagramRepository {
    type HexagramInfo;

    /// Fetch an `Option` possibly containing a ref to a `HexagramInfo` object. The ordering of
    /// objects in the repository should conform to a known sequence, which can be checked
    /// by calling `get_ordering`.
    /// _[KingWen](crate::HexagramOrdering::KingWen) Sequence is typical._
    fn get_by_number(&self, number: u8) -> Option<&Self::HexagramInfo>;
    /// Given a hexagram, fetch an `Option` possibly containing a ref to a `HexagramInfo` object.
    fn get_info_for_hexagram(&self, hexagram: &Hexagram) -> &Self::HexagramInfo;
    /// Check to see if this repository has been initialized. If `false`, the repository may fail
    /// to function correctly or even panic if other methods are called.
    fn get_is_initialized(&self) -> bool;
    /// Get the ordering sequence of this repository.
    /// _[KingWen](crate::HexagramOrdering::KingWen) Sequence is typical._
    fn get_ordering(&self) -> &HexagramOrdering;
    /// Do anything necessary to initialize this repository. Returns an empty `Result`
    /// if successful and returns a boxed error otherwise.
    fn initialize(&mut self) -> Result<(), Box<dyn Error>>;
}
