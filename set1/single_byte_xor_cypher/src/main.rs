use std::{fmt, num::ParseIntError};
use std::str;
use std::collections::HashMap;

use phf::{phf_map};

pub fn decode_hex(s: &str) -> Result<Vec<u8>, DecodeHexError> {
    // Creates a vector of bytes from a string of even hex values.

    // Return an error if the number of hex characters is odd since half
    // bytes are not considered.
    if s.len() %2 != 0 {
        Err(DecodeHexError::OddLength)
    } else {
        // Every two hex characters of the string, take the next two characters
        // and convert them from base16 to binary. Give a ParseIntError if map
        // fails.
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.into()))
            .collect()
    }
}

// Lookup table of hex characters.
const HEX_BYTES: &str = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f\
                         202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f\
                         404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f\
                         606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f\
                         808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f\
                         a0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebf\
                         c0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedf\
                         e0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff";


pub fn encode_hex(bytes: &[u8]) -> String {
    // Returns a hex string from a byte slice.

    // Let i represent double the byte value since 2 hex
    // characters represent 1 byte. Find index of byte in
    // hex lookup table.
    bytes
        .iter()
        .map(|&b| unsafe {
            let i = 2 * b as usize;
            HEX_BYTES.get_unchecked(i..i + 2)
        })
        .collect()
}

// Custom error type for DecodeHexError.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeHexError {
    OddLength,
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for DecodeHexError {
    fn from(e: ParseIntError) -> Self {
        DecodeHexError::ParseInt(e)
    }
}

impl fmt::Display for DecodeHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeHexError::OddLength => "input string has an odd number of bytes".fmt(f),
            DecodeHexError::ParseInt(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for DecodeHexError {}

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

pub fn is_alphabetic(c: u8) -> bool {
    if (91 > c && c > 64) || (123 > c && c > 96) {
        return true;
    } else {
        return false;
    };
}

fn calculate_frequency_quotient(decipher_attempt: &[u8]) -> f32 {
    let mut total_deviation = 0.0;
    let mut total_letters = 0;

    // Create a hashmap to count the occurrence frequency of each byte.
    let mut byte_counts: HashMap<u8, f32> = HashMap::new();
    for &byte in decipher_attempt {
        if is_alphabetic(byte) {
            *byte_counts.entry(byte.to_ascii_lowercase()).or_insert(0.0) += 1.0;
            total_letters += 1;
        }
    }

    for &byte in decipher_attempt {
        if is_alphabetic(byte) {
            let observed_count = byte_counts.get(&byte.to_ascii_lowercase()).unwrap_or(&0.0);
            let expected_count = ETAOIN_SHRDLU.get(&byte.to_ascii_lowercase()).unwrap_or(&0.0);
            total_deviation += (observed_count - expected_count).abs();
        }
    }

    // Normalize by dividing by the total number of letters
    if total_letters > 0 {
        total_deviation / total_letters as f32
    } else {
        std::f32::MAX // Avoid division by zero
    }
}

fn main() {
    let cypher_text: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let bytes = match decode_hex(cypher_text) {
        Ok(decoded) => decoded,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut cipher_key = 0;
    let mut text: Vec<u8> = Vec::new();
    let mut lowest_fq: f32 = 0.0;


    // Iterate through the total possible values of a single byte.
    for i in 0..=255 {
        // Apply XOR with byte value.
        let decipher_attempt: Vec<u8> = bytes.iter().map(|&b| b ^ i).collect();

        let fq = calculate_frequency_quotient(&decipher_attempt);

        if (fq < lowest_fq) || (lowest_fq == 0.0) {
            lowest_fq = fq;
            text = decipher_attempt;
            cipher_key = i;
        }

        println!("Key: {:x}, FQ: {}", i, fq);
    }

    // Convert Vec<u8> to String
    if let Ok(string_result) = String::from_utf8(text) {
        // Successfully converted to String
        let result_string = string_result;
        println!("Converted String: {}", result_string);
    } else {
        // Conversion failed
        println!("Failed to convert to String. Not valid UTF-8.");
    }
    
}
