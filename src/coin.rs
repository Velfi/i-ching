use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;

#[derive(Debug)]
pub enum Coin {
    Heads,
    Tails,
}

impl Distribution<Coin> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Coin {
        match rng.gen_bool(0.5) {
            false => Coin::Tails,
            true => Coin::Heads,
        }
    }
}