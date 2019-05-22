extern crate rand;
extern crate hex;
extern crate base64;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input the guess");

    let secret = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed {}", guess);
}

#[allow(dead_code)]
fn hex_to_base64(hexstring : &str) -> String {
    let result = hex::decode(hexstring);
    let bin = result.unwrap();
    let base64 = base64::encode(&bin);
    return base64;
}

#[allow(dead_code)]
fn fixed_xor(hex_left: &str, hex_right: &str) -> String {
    let left_bin = hex::decode(hex_left).unwrap();
    let right_bin = hex::decode(hex_right).unwrap();
    let mut result: Vec<u8> = Vec::new();
    for num in 0..left_bin.len() {
        result.push(left_bin[num]^right_bin[num]);
    }
    return hex::encode(result);
}

#[allow(dead_code)]
fn xor_decode(hex_left: &str, key: u8) -> String {
    let left_bin = hex::decode(hex_left).unwrap();
    let mut result: Vec<u8> = Vec::new();
    // let key_u8 = key as u8;
    for num in 0..left_bin.len() {
        result.push(left_bin[num]^key);
    }
    let plaintext = std::str::from_utf8(&result).unwrap();
    return String::from(plaintext);
}

#[allow(dead_code)]
fn get_score(message: &str) -> i32 {
    let iter = message.split_whitespace();
    let mut total = 1;
    for word in iter {
        total = total * word.len();
    }
    let score:i32 = (total) as i32;
    return score;
}

#[allow(dead_code)]
fn crack_xor(message: &str) -> String {
    let start = 'A' as u8;
    let end = 'z' as u8;
    let mut max = 0;
    let mut result = String::new();
    for key in start..end {
        let curr = xor_decode(message, key);
        let s = get_score(&curr);
        if s > max {
            max = s;
            result = curr;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let result = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        assert_eq!(result, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    fn test_fixed_xor() {
        let left = "1c0111001f010100061a024b53535009181c";
        let right = "686974207468652062756c6c277320657965";
        let result = fixed_xor(left, right);
        assert_eq!(result, "746865206b696420646f6e277420706c6179");
    }

    #[test]
    fn test_crack_xor() {
        let message = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let plaintext = crack_xor(message);
        assert_eq!(plaintext, "Cooking MC's like a pound of bacon");
    }
}
