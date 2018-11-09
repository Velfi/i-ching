#`iching`

This library contains methods for divination using the I Ching and also includes a CLI app
for making predictions in your terminal. The CLI app was inspired by the original
[ching(6)](http://cfcl.com/ching/man/) unix app.

The I Ching (a.k.a. the *Book of Changes*) is an ancient method of divination based on
cleromancy (assigning meaning to the generation of apparently random numbers.) Six numbers
between 6 and 9 are generated in order to create a hexagram, the meaning of which is
contained in the I Ching book.

You can find lots of great information on the 2000+ year history of the I Ching on
[Wikipedia](https://en.wikipedia.org/wiki/I_Ching)

## Installation
To install this crate as a CLI app, just run
```
cargo install iching
```
Once installed, you can access the CLI help screen like this:
```
iching --help
```
This project is a work in progress. If you find any issues, please submit them through Github.
## A simplified example:
```rust
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
                                
   // Print the hexagram info for the user
   print!("{}", hexagram_info);
```
