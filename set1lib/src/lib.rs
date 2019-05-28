extern crate hex;
extern crate base64;

const FREQUENCY:[f64;26] = [8.167, 1.492, 2.782, 4.253, 12.702, 2.228, 2.015, 
                            6.094, 6.966, 0.153, 0.772, 4.025, 2.406, 6.749,
                            7.507, 1.929, 0.095, 5.987, 6.327, 9.056,
                            2.758, 0.978, 2.360, 0.150, 1.974, 0.074]; 

#[allow(dead_code)]
fn hex_to_base64(hexstring : &str) -> String {
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
        result.push(left_bin[num]^right_bin[num]);
    }
    hex::encode(result)
}

#[allow(dead_code)]
fn xor_decode(hex_left: &str, key: u8) -> Option<String> {
    let left_bin = hex::decode(hex_left).unwrap();
    let mut result: Vec<u8> = Vec::new();
    // let key_u8 = key as u8;
    for num in 0..left_bin.len() {
        result.push(left_bin[num]^key);
    }
    let plaintext_opt = std::str::from_utf8(&result).ok();
    match plaintext_opt {
        Some(plaintext) => return Some(String::from(plaintext)),
        None => return None
    }
}

#[allow(dead_code)]
fn get_score(message: &str) -> i32 {
    let iter = message.split_whitespace();
    let mut total = 1;
    for word in iter {
        total = total * word.len();
    }
    let score:i32 = (total) as i32;
    score
}


pub fn crack_xor(message: &str) -> (i32, String) {
    let start = '0' as u8;
    let end = 'z' as u8;
    let mut max = 0;
    let mut result = String::new();
    for key in start..end {
        let curr = xor_decode(message, key);
        if curr == None {
            continue;
        }
        let curr_msg = curr.unwrap();
        let s = get_score(&curr_msg);
        if s > max {
            max = s;
            result = curr_msg;
            // println!("debug {} {}", max, result);
        }
    }
    (max, result)
}


pub fn repeating_key_xor(key:&str, message:&str) -> String {
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();
    let message_bytes = message.as_bytes();
    let mut plaintext_bytes:Vec<u8> = Vec::new();
    for (i, data) in message_bytes.iter().enumerate() {
        let k = i % key_len;
        plaintext_bytes.push(data^key_bytes[k]);
    }
    let result = hex::encode(plaintext_bytes);
    result
}

pub fn hamming_distance(left:&[u8], right:&[u8]) -> u32 {
    let mut distance:u32 = 0;
    for (i, data) in left.iter().enumerate() {
        let mut result:u8 = data^right[i];
        while result > 0 {
            let d = result&1;
            if d == 1  {
                distance += 1;
            }
            result = result >> 1;
        }
    }
    distance
}

pub fn get_frequency(size:u32) -> Vec<i32> {
    let result: Vec<i32> = Vec::new();
    result
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
        let plaintext = crack_xor(message).1;
        assert_eq!(plaintext, "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn test_repeating_key_xor() {
        let key = "ICE";
        let message1 = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let result = repeating_key_xor(key, message1);
        assert_eq!(result, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }

    #[test]
    fn test_hamming_distance() {
        let left = "this is a test".as_bytes();
        let right = "wokka wokka!!!".as_bytes();
        let result = hamming_distance(left, right);
        assert_eq!(result, 37);
    }
}
