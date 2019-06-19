extern crate set1lib;

use std::env;
use std::fs;

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
    let result = set1lib::decrypt(&content_b, key).expect("cannot decrypt");
    let plaintext = String::from_utf8(result);
    println!("{}", plaintext.unwrap());
}
