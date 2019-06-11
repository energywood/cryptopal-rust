extern crate set1lib;
extern crate crypto;

use std::env;
use std::fs;
use data_encoding::HEXLOWER;
// use crypto::{symmetriccipher, aes, blockmodes};

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = contents.lines();
    let mut min_dis = std::u32::MAX;
    let mut min_line = "";
    let mut min_line_no = 0;
    for (n, line) in lines.enumerate() {
        let bin = HEXLOWER.decode(line.as_bytes()).expect("invalid hex");
        let mut hamming_distance = 0;
        let blocks = bin.len()/32 - 1;
        for i in 0..blocks {
            let first = &bin[i * 32..(i + 1) * 32];
            let second = &bin[(i + 1) * 32..(i + 2) * 32];
            hamming_distance += set1lib::hamming_distance(first, second);
        }
        // println!("{}, {}", n, hamming_distance);
        if hamming_distance < min_dis {
            min_dis = hamming_distance;
            min_line = line;
            min_line_no = n;
        }
    }
    println!("{}, {}, {}", min_line_no, min_line, min_dis);
}
