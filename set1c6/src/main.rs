extern crate base64;
extern crate set1lib;
use std::collections::BinaryHeap;
use std::cmp::PartialOrd;
use std::cmp::Ord;
use std::cmp::PartialEq;
use std::fs;
use std::env;


#[derive(Debug, PartialOrd, Ord)]
struct KeySizeEstimate {
    min_dis:i32,
    key_size:usize
}

impl PartialEq for KeySizeEstimate  {
    fn eq(&self, other: &Self) -> bool {
        self.min_dis == other.min_dis
    }
}

impl Eq for KeySizeEstimate {}




fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents = contents.lines();
    let mut contents_trim = String::new();
    for line in contents {
        contents_trim.push_str(line);
    }
    let content_b = base64::decode(&contents_trim).expect("decoding error");
    let result = key_size_estimate(&content_b[..]);
    println!("result {:?}", result);
    
}

fn key_size_estimate(contents:&[u8]) -> BinaryHeap<KeySizeEstimate> {
    let mut result = BinaryHeap::new();
    for key_size in 2..40 {
        let first = &contents[0..key_size];
        let second = &contents[key_size..2*key_size];
        let hamming_distance = set1lib::hamming_distance(first, second);
        let ham_dis_norm:i32 = (hamming_distance*100 / key_size as u32) as i32;
        result.push(KeySizeEstimate{min_dis: -ham_dis_norm, key_size:key_size});
    }

    result
}
