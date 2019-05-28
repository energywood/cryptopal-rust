extern crate base64;
extern crate set1lib;
use std::collections::BinaryHeap;
use std::cmp::PartialOrd;
use std::cmp::PartialEq;
use std::fs;
use std::env;


#[derive(PartialEq, PartialOrd)]
struct KeySizeEstimate {
    min_dis:f64,
    key_size:usize
}


fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

}

fn key_size_estimate(contents:&str) -> BinaryHeap<KeySizeEstimate> {
    let result = BinaryHeap::new();
    for key_size in 2..40 {
        let first = &contents[0..key_size];
        let second = &contents[key_size..2*key_size];
        let hamming_distance = set1lib::hamming_distance(first, second);
        let ham_dis_norm = -hamming_distance as f64 / key_size as f64;
        result.push(KeySizeEstimate{min_dis: ham_dis_norm, key_size:key_size});
    }

    result
}
