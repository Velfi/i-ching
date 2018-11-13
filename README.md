# iching

[![Crates.io][crates-badge]][crates-link] [![Documentation][docs-badge]][docs-link] [![MIT/Apache][license-badge]][license-link] ![Lines of Code][loc-badge]

[crates-badge]: https://img.shields.io/crates/v/iching.svg?maxAge=86400
[docs-badge]: https://docs.rs/iching/badge.svg?maxAge=86400
[docs-link]: https://docs.rs/iching/
[license-badge]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg?maxAge=86400
[loc-badge]: https://tokei.rs/b1/github/velfi/i-ching?category=code
[crates-link]: https://crates.io/crates/iching/
[license-link]: COPYING

**This crate requires beta or nightly until the 2018 edition becomes stable. The
auto docs may not build correctly until then either. That being said, running
`cargo doc --open` will work just fine.**

This library contains methods for divination using the I Ching and also includes a CLI app
for making predictions in your terminal. The CLI app was inspired by the original
[ching(6)](http://cfcl.com/ching/man/) unix app.

The I Ching (a.k.a. the *Book of Changes*) is an ancient method of divination based on
cleromancy (assigning meaning to the generation of apparently random numbers.) Six numbers
between 6 and 9 are generated in order to create a hexagram, the meaning of which is
contained in the I Ching book.

You can find lots of great information on the 2000+ year history of the I Ching on
[Wikipedia](https://en.wikipedia.org/wiki/I_Ching)

## Using the CLI app
To install this crate as a CLI app, just run
```
cargo install iching
```
*This assumes that you have Rust installed on your machine.*

Once installed, you can access the help screen with the `--help` flag:
```
iching --help
```
This project is a work in progress. If you find any issues, please submit them through Github.
## Using the crate
```rust
fn main() {
   // Implementing the HexagramRepository trait is the most complex
   // aspect of using this library. See the included CLI app for an
   // example implementation called HexagramJson.
   let mut hexagrams: HexagramRepository = HexagramJson::new();
   
   // Don't forget to initialize repository after construction, else
   // it could fail to work or even panic.
   hexagrams.initialize().expect("Initialization of hexagrams has failed");
   
   // Create a new random hexagram.
   let new_hexagram = Hexagram::from_coin_tosses();
   
   // Get the number of the hexagram according to changing lines and ordering
   let hexagram_number = new_hexagram.as_number(false, HexagramOrdering::KingWen);
   
   // Fetch the hexagram's info from the repository that was initialized earlier.
   let hexagram_info = hexagrams.get_by_number(hexagram_number)
                                .expect("Failed to get hexagram info by number (pre)");
                                
   // Print the hexagram info
   print!("{}", hexagram_info);
}
```

## License
iching is free and open source software distributed under the terms of both
the [MIT License][mit-license] and the [Apache License 2.0][apache-license].

[mit-license]: license/LICENSE-MIT
[apache-license]: license/LICENSE-APACHE

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.