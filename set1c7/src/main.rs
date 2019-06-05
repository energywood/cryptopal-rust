extern crate crypto;
extern crate rand;

use crypto::{ symmetriccipher, buffer, aes, blockmodes, ed25519 };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

use rand::{ Rng, OsRng };


fn main() {
    let mut rng = OsRng::new().unwrap();
    let mut seed:[u8;32] = [0;32];
    rng.fill_bytes(&mut seed);
    let (sk, pk) = ed25519::keypair(&seed);
    let sig = ed25519::signature(b"Hello world", &sk);
    let res = ed25519::verify(b"Hello world", &pk, &sig);
    println!("Hello, world! {}", res);
}
