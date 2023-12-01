use base64::{Engine as _, engine::general_purpose};
use std::{fmt, num::ParseIntError};

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

pub fn printb64(result: Result<Vec<u8>, DecodeHexError>) {
    // Print result of base64 encoding unless an error occured
    // when decoding.
    match result {
        Ok(n)  => println!("{}", general_purpose::STANDARD_NO_PAD.encode(n)),
        Err(e) => println!("Error: {}", e),
    }
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


fn main() {
    // Hex string given in challenge 1.
    let hex_string: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    // Prints the base64 converted hex string.
    printb64(decode_hex(hex_string));

}
