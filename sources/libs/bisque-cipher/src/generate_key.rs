use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn generate_key() -> [u8; 16] {
    let mut seed: [u8; 32] = [0; 32];
    let mut rng = StdRng::from_entropy();
    rng.fill(&mut seed);

    let mut key = [0; 16];
    key.copy_from_slice(&seed[..16]);
    key
}
