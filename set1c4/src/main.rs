extern crate hex;

use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();

    for (i, line) in lines.enumerate() {
        if i == 100 {
            let result = hex::decode(line).unwrap();
            let text = String::from_utf8(result).unwrap();
        }
        println!("line:{} {}", i, line);
    }
}
