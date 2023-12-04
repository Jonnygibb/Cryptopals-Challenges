use std::str;
use std::collections::HashMap;

use hex;
use phf::{phf_map};

// Frequency of letter appearance in concise oxford dictionary(9th edition, 1995).
static ETAOIN_SHRDLU: phf::Map<u8, f32> = phf_map! {
    b'e' => 11.1607,
    b'a' => 8.4966,
    b'r' => 7.5809,
    b'i' => 7.5448,
    b'o' => 7.1635,
    b't' => 6.9509,
    b'n' => 6.6544,
    b's' => 5.7351,
    b'l' => 5.4893,
    b'c' => 4.5388,
    b'u' => 3.6308,
    b'd' => 3.3844,
    b'p' => 3.1671,
    b'm' => 3.0129,
    b'h' => 3.0034,
    b'g' => 2.4705,
    b'b' => 2.0720,
    b'f' => 1.8121,
    b'y' => 1.7779,
    b'w' => 1.2899,
    b'k' => 1.1016,
    b'v' => 1.0074,
    b'x' => 0.2902,
    b'z' => 0.2722,
    b'j' => 0.1965,
    b'q' => 0.1962,
};

pub fn is_control(u: u8) -> bool {
    u < 0x20 || u == 0x7F
}

pub fn is_alphabetic(u: u8) -> bool {
    (u >= 0x41 && u <= 0x5A) || (u >= 0x61 && u <= 0x7A)
}

pub fn single_byte_xor(stream: &Vec<u8>, key: u8) -> Vec<u8> {
    return stream.iter().map(|b| b ^ key).collect();
}

pub fn calculate_character_counts(stream: &Vec<u8>) -> HashMap<u8, f32> {
    let mut char_counts = HashMap::<u8, f32>::new();
    for byte in stream {
        if is_alphabetic(*byte) {
            *char_counts.entry(byte.to_ascii_lowercase()).or_insert(0.0) += 1.0;
        } else {
            continue;
        }
        
    }
    return char_counts;
}

fn calculate_fitting_quotient(stream: &Vec<u8>) -> f32{
    let b = stream.as_slice();

    // If bytes are not ascii characters, return maximum value.
    if !b.is_ascii() {
        return std::f32::MAX;
    } else {
        let char_counts = calculate_character_counts(stream);
        let length = b.len() as f32;

        // Calculate the fitting quotient
        let fitting_quotient = ETAOIN_SHRDLU.entries.iter().fold(0.0, |a, &(c, frequency)| {
            let expected_count = frequency / 100.0 * length;
            let actual_count = char_counts.get(&c).unwrap_or(&0.0);
            a + (expected_count - actual_count).powi(2)
        });

        return fitting_quotient;
    }


}

fn main() {
    // Encoded text given by the challenge.
    let cipher_text: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    // Decode the text into bytes.
    let bytes = match hex::decode(&cipher_text) {
        Ok(decoded) => decoded,
        Err(error) => panic!("Error: {}", error),
    };

    let mut lowest_fq = 0.0;
    let mut cipher_key = 0;
    let mut text: Vec<u8> = vec!(0);

    for key in 0..=255 {
        // Perform the single byte XOR.
        let decipher_attempt = single_byte_xor(&bytes, key);

        // Find the occurances of character in the deciphered stream.
        // let char_counts: HashMap<u8, f32> = calculate_character_counts(&decipher_attempt);
        
        // Iterate over the bytes of byte stream. 
        // If alphabetic or compare with ETAOIN_SHRDLU
        // If space, tab, newline do nothing.
        // If control character, return large value
        // if none of above, 
        let fitting_quotient = calculate_fitting_quotient(&decipher_attempt);

        if fitting_quotient < lowest_fq || lowest_fq == 0.0 {
            lowest_fq = fitting_quotient;
            cipher_key = key;
            text = decipher_attempt;
        }
        
    }

    println!("Key: {:x}, fitting quotient {}", cipher_key, lowest_fq);
    let deciphered_text_chars: Vec<char> = single_byte_xor(&text, cipher_key).iter().map(|&byte| byte as char).collect();
    let deciphered_text: String = deciphered_text_chars.into_iter().collect();
    println!("{}", deciphered_text);
    
}
