use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::num::ParseIntError;
use std::collections::HashSet;


/// Reads lines from a file.
/// 
/// Returns an iterator of lines from the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// Use where to constraint that P must be a path.
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


/// Creates a vector of bytes from a string of even hex values.
pub fn decode_hex(s: &str) -> Result<Vec<u8>, DecodeHexError> {
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


fn main() {
    if let Ok(lines) = read_lines("./ciphertexts.txt") {
        // If the iterator is valid, iterate over the lines.
        for line in lines.map_while(Result::ok) {
            if let Ok(bytes) = decode_hex(&line) {
                // If the line can be decoded from hex to bytes, assert
                // that the bytes can be split into blocks of 16 bytes.
                assert!(bytes.len() % 16 == 0);
                let mut set = HashSet::new();

                // Insert blocks of 16 bytes into a hashmap.
                for i in (0..bytes.len()).step_by(16) {
                    set.insert(&bytes[i..i+16]);
                }

                // Detect whether any duplicate blocks existed.
                if set.len() != bytes.len() / 16 {
                    println!("AES ECB Detected!\nHex String: {}", line);
                    break;
                }
            }
        }
    }
}
