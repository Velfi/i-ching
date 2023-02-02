//! This library contains methods for divination using the I Ching. The related CLI app was inspired
//! by the original [ching(6)](http://cfcl.com/ching/man/) unix program.
//!
//! The I Ching (a.k.a. the *Book of Changes*) is an ancient method of divination based on
//! cleromancy (assigning meaning to the generation of apparently random numbers.) Six numbers
//! between 6 and 9 are generated in order to create a hexagram, the meaning of which is
//! contained in the I Ching book.
//!
//! You can find lots of great information on the 2000+ year history of the I Ching on
//! [Wikipedia](https://en.wikipedia.org/wiki/I_Ching)
//!
//! ## Installing and running the CLI app
//!
//! To install the CLI app, just run
//!
//! ```sh
//! cargo install iching
//! ```
//!
//! Once installed, you can access the help screen by running the CLI with no arguments.
//!
//! If you find any issues, please submit them through Github.
//!
//! # A simplified example of using this library:
//!
//! ```no_run
//! use iching::{Hexagram, HexagramOrdering, HexagramRepository, divination_method::DivinationMethod};
//! # #[derive(Debug)]
//! # struct HexagramExampleInfo;
//! # struct HexagramExampleRepository;
//! # impl HexagramExampleRepository {
//! #     fn new() -> Self { HexagramExampleRepository }
//! #     fn initialize(&mut self) -> Result<(), Box<(dyn std::error::Error + 'static)>> { Ok(()) }
//! # }
//! # impl HexagramRepository for HexagramExampleRepository {
//! #     type HexagramInfo = HexagramExampleInfo;
//! #
//! #     fn initialize(&mut self) -> Result<(), Box<(dyn std::error::Error + 'static)>> { Ok(()) }
//! #     fn get_is_initialized(&self) -> bool { true }
//! #     fn get_ordering(&self) -> &HexagramOrdering { &HexagramOrdering::KingWen }
//! #     fn get_by_number(&self, _number: u8) -> Option<&Self::HexagramInfo> { None }
//! #     fn get_info_for_hexagram(&self, _hexagram: &Hexagram) -> &Self::HexagramInfo { &HexagramExampleInfo }
//! # }
//!
//! // Implementing the HexagramRepository trait is the most complex
//! // aspect of using this library. See the included CLI app for an
//! // example implementation called HexagramExampleRepository.
//! let hexagrams: &mut dyn HexagramRepository<HexagramInfo = HexagramExampleInfo> = &mut HexagramExampleRepository::new();
//!
//! // Don't forget to initialize repository after construction, else
//! // it could fail to work or even panic.
//! hexagrams.initialize().expect("Initialization of hexagrams has failed");
//!
//! // Create a new random hexagram.
//! let new_hexagram = Hexagram::new_random(DivinationMethod::AncientYarrowStalk);
//!
//! // Fetch the hexagram's info from the repository that was initialized earlier.
//! let hexagram_info = hexagrams.get_info_for_hexagram(&new_hexagram);
//!
//! // Print the hexagram info for the user
//! print!("{hexagram_info:?}");
//! ```

/// Types related to the various methods of divination.
pub mod divination_method;
/// `Hexagram`s are used for divination in the I Ching.
/// This module contains hexagram generation and management tools.
pub mod hexagram;
/// Contains traits for implementing generic repositories of hexagram meanings.
pub mod hexagram_repository;
/// `Line`s are the building blocks of `Hexagram`s and `Trigram`s.
/// This module contains ways of randomly generating lines
pub mod line;
/// A collection of various I-Ching related symbols, in unicode or ASCII-art form.
pub mod symbols;
/// `Trigram`s are the building blocks of `Hexagrams`.
/// This module contains trigram generation and management tools.
pub mod trigram;

pub use hexagram::{Hexagram, HexagramOrdering};
pub use hexagram_repository::HexagramRepository;
