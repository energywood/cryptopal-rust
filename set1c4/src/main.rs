extern crate hex;
extern crate set1lib;
use std::env;
use std::fs;


fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();

    let mut line_num = 0;
    let mut max = 0;
    let mut result = String::new();
    for (i, line) in lines.enumerate() {
        let bin = hex::decode(line).expect("invalid hex");
        let (_, text, score) = set1lib::crack_xor(&bin);
        // println!("line:{} {}", i, text);
        if score > max {
            max = score;
            result = text;
            line_num = i;
        }
    }
    println!("result: {} {} {}", line_num, max, result);
}
