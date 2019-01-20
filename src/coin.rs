use rand::{distributions::Distribution, distributions::Standard, Rng};

/// Represents the possible states of a coin toss.
/// Used for randomly generating [`Line`](../line/enum.Line.html)s.
#[derive(Debug)]
pub enum Coin {
    Heads,
    Tails,
}

impl Distribution<Coin> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Coin {
        if rng.gen_bool(0.5) {
            Coin::Heads
        } else {
            Coin::Tails
        }
    }
}
