use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn generate() -> [u8; 32 * 2] {
    let mut seed = [0; 32 * 2];
    let mut rng = StdRng::from_entropy();
    rng.fill(&mut seed);
    seed
}
