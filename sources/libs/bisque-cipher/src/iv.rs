use crate::seed;

pub struct Iv([u8; Iv::SIZE]);

impl Iv {
    const SIZE: usize = 16;

    pub fn generate() -> Self {
        let seed = seed::generate();
        let mut iv = [0; Self::SIZE];
        iv.copy_from_slice(&seed[..Self::SIZE]);
        Iv(iv)
    }

    pub fn as_bytes(&self) -> &[u8; Self::SIZE] {
        &self.0
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self(*bytes)
    }
}

impl From<&[u8; 16]> for Iv {
    fn from(bytes: &[u8; 16]) -> Self {
        Self(*bytes)
    }
}
