extern crate crypto;
extern crate rand;

use std::env;
use std::fs;
use crypto::{ symmetriccipher, buffer, aes, blockmodes, ed25519 };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

use rand::{ Rng, OsRng };

fn decrypt(encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::ecb_decryptor(
            aes::KeySize::KeySize128,
            key,
            blockmodes::NoPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut spec = data_encoding::BASE64.specification();
    spec.ignore.push_str("\n");
    let base64 = spec.encoding().unwrap();
    let content_b = base64.decode(contents.as_bytes()).expect("invalid b64");


    let key = b"YELLOW SUBMARINE";
    let result = decrypt(&content_b, key).expect("cannot decrypt");
    let plaintext = String::from_utf8(result);
    println!("{}", plaintext.unwrap());
}
