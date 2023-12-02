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
static ETAOIN_SHRDLU: phf::Map<char, f32> = phf_map! {
    'e' => 11.1607,
    'a' => 8.4966,
    'r' => 7.5809,
    'i' => 7.5448,
    'o' => 7.1635,
    't' => 6.9509,
    'n' => 6.6544,
    's' => 5.7351,
    'l' => 5.4893,
    'c' => 4.5388,
    'u' => 3.6308,
    'd' => 3.3844,
    'p' => 3.1671,
    'm' => 3.0129,
    'h' => 3.0034,
    'g' => 2.4705,
    'b' => 2.0720,
    'f' => 1.8121,
    'y' => 1.7779,
    'w' => 1.2899,
    'k' => 1.1016,
    'v' => 1.0074,
    'x' => 0.2902,
    'z' => 0.2722,
    'j' => 0.1965,
    'q' => 0.1962,
};

fn main() {
    let cypher_text: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let bytes = match decode_hex(cypher_text) {
        Ok(decoded) => decoded,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };


    // Iterate through the total possible values of a single byte.
    for i in 0..=255 {
        // Apply XOR with byte value.
        let decipher_attempt: Vec<u8> = bytes.iter().map(|&b| b ^ i).collect();

        let char_vec: Vec<char> = decipher_attempt.iter().map(|&b| b as char).collect();

        // Count the occurrences of each character.
        let mut letter_counts: HashMap<char, usize> = HashMap::new();
        for &c in &char_vec {
            *letter_counts.entry(c).or_insert(0) += 1;
        }

        println!("Key: {:x}, Counts: {:?}", i, letter_counts);
        
        let fq = 0;

        // TO DO: Frequency quotient code
    }
    
}
