extern crate data_encoding;
extern crate set1lib;
use std::cmp::PartialEq;
use std::collections::BinaryHeap;
use std::env;
use std::fs;

#[derive(Debug, PartialOrd, Ord)]
struct KeySizeEstimate {
    min_dis: i32,
    key_size: usize,
}

impl PartialEq for KeySizeEstimate {
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
    let mut spec = data_encoding::BASE64.specification();
    spec.ignore.push_str("\n");
    let base64 = spec.encoding().unwrap();
    let content_b = base64.decode(contents.as_bytes()).expect("invalid b64");

    let key_size_list = key_size_estimate(&content_b[..]);
    let key_size = key_size_list.peek().unwrap().key_size;
    println!("result {:?}", key_size_list);
    let key = find_key(&content_b, key_size);
    println!("key: {}", String::from_utf8(key.clone()).unwrap());
    let plaintext = set1lib::repeating_key_xor_str(&key, &content_b);
    println!("{}", plaintext);
}

fn key_size_estimate(contents: &[u8]) -> BinaryHeap<KeySizeEstimate> {
    let mut result = BinaryHeap::new();
    for key_size in 2..41 {
        let k = contents.len() / (key_size * 2);
        let mut hamming_distance = 0;
        for i in 0..k {
            let first = &contents[i * key_size..(i + 1) * key_size];
            let second = &contents[(i + 1) * key_size..(i + 2) * key_size];
            hamming_distance += set1lib::hamming_distance(first, second);
        }
        let hamming_distance = hamming_distance / (k as u32);
        let ham_dis_norm: i32 = (hamming_distance * 100 / key_size as u32) as i32;
        result.push(KeySizeEstimate {
            min_dis: -ham_dis_norm,
            key_size: key_size,
        });
    }

    result
}

fn find_key(contents: &[u8], key_size: usize) -> Vec<u8> {
    let mut key: Vec<u8> = Vec::new();
    for i in 0..key_size {
        let transpose: Vec<_> = contents
            .iter()
            .enumerate()
            .filter(|&(j, _)| j % key_size == i)
            .map(|(_, e)| *e)
            .collect();

        let (k, _, _) = set1lib::crack_xor(&transpose);
        key.push(k as u8);
    }
    key
}
