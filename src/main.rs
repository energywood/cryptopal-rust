// extern crate rand;
extern crate hex;
extern crate base64;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
