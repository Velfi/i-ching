use rand::{
    distributions::Distribution,
    distributions::Standard,
    Rng,
};

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