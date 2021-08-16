use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::rand;

// copied from https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=beb07e7b716d4828f2f4de22a8cb3e2f
pub fn hello() {
    let mut data = Vec::from(&make_key() as &[u8]); // make some data
    data.extend(&make_key()); // ensure the data is realistically long

    let key = make_key(); // make a key
    println!("key = {:?}", key); // print the key
    println!("{:?}", data); // print the original data

    encrypt(key, &mut data); // encrypt the data
    println!("{:?}", data); // print the encrypted data

    decrypt(key, &mut data); // decrypt the data
    println!("{:?}", data); // print the decrypted data
}

/// AES-256 has 256-bit keys
type Key = [u8; 256 / 8];

/// we need an encryption key
fn make_key() -> Key {
    // we need an UnboundKey for doing crypto
    let rng = rand::SystemRandom::new(); // this has SecureRandom, which rand::generate wants
    rand::generate(&rng).unwrap().expose() // generate the key
}

fn encrypt(key: Key, data: &mut Vec<u8>) {
    let key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, &key).unwrap()); // not sure why it's less safe but it has a simpler API
    let nonce = Nonce::assume_unique_for_key([0u8; 12]); // this is probably a bad idea
    key.seal_in_place_append_tag(nonce, Aad::empty(), data)
        .unwrap(); // I think this does encryption
}

fn decrypt(key: Key, data: &mut Vec<u8>) {
    let key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, &key).unwrap());
    let nonce = Nonce::assume_unique_for_key([0u8; 12]);
    key.open_in_place(nonce, Aad::empty(), data).unwrap(); // I think this does decryption
    data.truncate(data.len() - AES_256_GCM.tag_len()); // remove the garbage on the end
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
