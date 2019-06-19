extern crate base64;
extern crate hex;

const FREQUENCY: [f64; 26] = [
    8.167, 1.492, 2.782, 4.253, 12.702, 2.228, 2.015, 6.094, 6.966, 0.153, 0.772, 4.025, 2.406,
    6.749, 7.507, 1.929, 0.095, 5.987, 6.327, 9.056, 2.758, 0.978, 2.360, 0.150, 1.974, 0.074,
];

const AVG_WORD_SIZE: usize = 5;

#[allow(dead_code)]
fn hex_to_base64(hexstring: &str) -> String {
    let result = hex::decode(hexstring);
    let bin = result.unwrap();
    let base64 = base64::encode(&bin);
    base64
}

#[allow(dead_code)]
fn fixed_xor(hex_left: &str, hex_right: &str) -> String {
    let left_bin = hex::decode(hex_left).unwrap();
    let right_bin = hex::decode(hex_right).unwrap();
    let mut result: Vec<u8> = Vec::new();
    for num in 0..left_bin.len() {
        result.push(left_bin[num] ^ right_bin[num]);
    }
    hex::encode(result)
}

#[allow(dead_code)]
fn xor_decode(bin: &[u8], key: u8) -> Option<String> {
    let mut result: Vec<u8> = Vec::new();
    for num in 0..bin.len() {
        result.push(bin[num] ^ key);
    }
    let plaintext_opt = std::str::from_utf8(&result).ok();
    match plaintext_opt {
        Some(plaintext) => return Some(String::from(plaintext)),
        None => return None,
    }
}

#[allow(dead_code)]
fn get_score(message: &str) -> i32 {
    let iter = message.split_whitespace();
    let mut total = 1;
    for word in iter {
        total = total * word.len();
    }
    let score: i32 = (total) as i32;
    -score
}

fn get_score_v2(message: &str) -> i32 {
    let mut frequency = get_frequency(message.len());
    let mut non_letter = 0;
    let mut space: i32 = 0;
    for &data in message.to_ascii_lowercase().as_bytes() {
        let idx = (data as i32) - ('a' as i32);
        if idx >= 0 && idx < 26 {
            let idx = idx as usize;
            frequency[idx] = frequency[idx] - 1;
        } else if data == (' ' as u8) {
            space += 1;
        } else {
            non_letter += 1;
        }
    }

    let variance: Vec<_> = frequency.iter().map(|x| x * x).collect();
    let mut result = variance.iter().sum();
    result += non_letter * non_letter;
    space = (message.len() / AVG_WORD_SIZE) as i32 - space;
    result += (space * space) as i32;
    result
}

pub fn crack_xor(message: &[u8]) -> (char, String, i32) {
    let start = ' ' as u8;
    let end = '~' as u8;
    let mut k = ' ';
    let mut min = std::i32::MAX;
    let mut result = String::new();
    for key in start..end {
        let curr = xor_decode(message, key);
        if curr == None {
            continue;
        }
        let curr_msg = curr.unwrap();
        let s = get_score_v2(&curr_msg);
        if s < min {
            min = s;
            k = key as char;
            result = curr_msg;
            // println!("debug {} {}", max, result);
        }
    }
    (k, result, min)
}

pub fn repeating_key_xor_str(key: &[u8], message: &[u8]) -> String {
    let plaintext_bytes = repeating_key_xor(key, message);
    let plaintext = String::from_utf8(plaintext_bytes).expect("invalid utf8");
    plaintext
}

pub fn repeating_key_xor_hex(key: &str, message: &str) -> String {
    let key_bytes = key.as_bytes();
    let message_bytes = message.as_bytes();
    let plaintext_bytes = repeating_key_xor(key_bytes, message_bytes);
    let result = hex::encode(plaintext_bytes);
    result
}

fn repeating_key_xor(key_bytes: &[u8], message_bytes: &[u8]) -> Vec<u8> {
    let key_len = key_bytes.len();
    let mut plaintext_bytes: Vec<u8> = Vec::new();
    for (i, data) in message_bytes.iter().enumerate() {
        let k = i % key_len;
        plaintext_bytes.push(data ^ key_bytes[k]);
    }
    plaintext_bytes
}

pub fn hamming_distance(left: &[u8], right: &[u8]) -> u32 {
    let mut distance: u32 = 0;
    for (i, data) in left.iter().enumerate() {
        let mut result: u8 = data ^ right[i];
        while result > 0 {
            let d = result & 1;
            if d == 1 {
                distance += 1;
            }
            result = result >> 1;
        }
    }
    distance
}

pub fn get_frequency(size: usize) -> Vec<i32> {
    let frequency: Vec<i32> = FREQUENCY
        .iter()
        .map(|x| (x / 100.0 * size as f64) as i32)
        .collect();
    frequency
}

pub fn pkcs7padding(data: &[u8], block: usize) -> Vec<u8> {
    let size = data.len();
    let even = block*2;
    let pad = (even - size%even)%even;
    let mut result = Vec::with_capacity(size + pad);
    for b in data {
        result.push(*b);
    }
    for _ in 0..pad {
        result.push(pad as u8);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let result = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        assert_eq!(
            result,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
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
        let plaintext = crack_xor(&hex::decode(message).unwrap()).1;
        assert_eq!(plaintext, "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn test_repeating_key_xor() {
        let key = "ICE";
        let message1 =
            "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let result = repeating_key_xor_hex(key, message1);
        assert_eq!(result, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }

    #[test]
    fn test_hamming_distance() {
        let left = "this is a test".as_bytes();
        let right = "wokka wokka!!!".as_bytes();
        let result = hamming_distance(left, right);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_pkcs_padding() {
        let message = b"YELLOW SUBMARINE";
        let exp = b"YELLOW SUBMARINE\x04\x04\x04\x04";
        let result = pkcs7padding(message, 10);
        assert_eq!(result, exp);
    }
}
