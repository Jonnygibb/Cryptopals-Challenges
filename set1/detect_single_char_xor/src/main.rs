use phf::phf_map;
use std::{io, fs};
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

// Source:
// Lee, E. Stewart. "Essays about Computer Security" (PDF). University of Cambridge Computer Laboratory. p. 181.
static EXPECTED_FREQUENCIES: phf::Map<u8, f64> = phf_map! {
    b' ' => 0.1217, // Whitespace
    b'.' => 0.0657,  // Others
    b'a' => 0.0609,
    b'b' => 0.0105,
    b'c' => 0.0284,
    b'd' => 0.0292,
    b'e' => 0.1136,
    b'f' => 0.0179,
    b'g' => 0.0138,
    b'h' => 0.0341,
    b'i' => 0.0544,
    b'j' => 0.0024,
    b'k' => 0.0041,
    b'l' => 0.0292,
    b'm' => 0.0276,
    b'n' => 0.0544,
    b'o' => 0.0600,
    b'p' => 0.0195,
    b'q' => 0.0024,
    b'r' => 0.0495,
    b's' => 0.0568,
    b't' => 0.0803,
    b'u' => 0.0243,
    b'v' => 0.0097,
    b'w' => 0.0138,
    b'x' => 0.0024,
    b'y' => 0.0130,
    b'z' => 0.0003,
};

pub fn get_character_counts(ciphertext: &[u8]) -> HashMap<u8, f64> {
    let mut char_counts: HashMap<u8, f64> = HashMap::new();
    for &char in ciphertext.iter() {
        
        if is_control(char) {
            continue;
        }  

        let key = if is_alphabetic(char) {
            char.to_ascii_lowercase()
        } else if char == b' ' || char == b'\t' {
            b' '
        } else {
            b'.'
        };

        let char_count = char_counts.entry(key).or_insert(0f64);
        *char_count += 1f64;
    }

    char_counts
}

pub fn chi_squared_test(ciphertext: &[u8]) -> f64 {

    let observed_freq = get_character_counts(ciphertext);
    let mut chi_squared = 0.0;

    for (&letter, &observation) in observed_freq.iter() {
        let expected = EXPECTED_FREQUENCIES.get(&letter).unwrap_or(&0f64) * ciphertext.len() as f64;
        chi_squared += ((observation as f64 - expected).powi(2)) / expected;
    }

    return chi_squared

}

pub fn xor_by_single_byte(stream: &[u8], byte: u8) -> Vec<u8>{
    return stream.iter().map(|b| b ^ byte).collect();
}

pub fn is_alphabetic(byte: u8) -> bool {
    (byte >= 0x41 && byte <= 0x5A) || (byte >= 0x61 && byte <= 0x7A)
}

pub fn is_control(u: u8) -> bool {
    u < 0x20 || u == 0x7F
}

pub fn calculate_score(ciphertext: &[u8]) -> f64 {
    
    // If any bytes in the ciphertext are non-ascii, return max score.
    if !ciphertext.is_ascii() {
        return std::f64::MAX;
    }

    // If any bytes in the ciphertext are control chars and not a newline,
    // return max score.
    if ciphertext.iter().any(|&c| is_control(c) && c != b'\n') {
        return std::f64::MAX;
    }

    let chi_square_stat = chi_squared_test(ciphertext);

    return chi_square_stat
}


fn read_file_lines(filename: &str) -> io::Result<Vec<Vec<u8>>> {
    // Open the file and create a buffered reader for efficient line reading.
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);

    // Collect lines into a Vec<Vec<u8>>, removing newline characters.
    let lines: Vec<Vec<u8>> = reader
        .lines()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect();

    Ok(lines)
}


pub fn detect_single_byte_xor(lines: &Vec<Vec<u8>>) {
        // Estblish variables to hold the lowest scoring key attempt.
        let mut cipher_key: u8 = 0;
        let mut lowest_score: f64 = std::f64::MAX;
        let mut decrypted_string: Vec<u8> = Vec::new();
    
        // Iterate over the lines of the file.
        for line in lines {

            // Convert from hex String to Vec<u8>.
            let raw_bytes = match hex::decode(line) {
                Ok(decoded) => decoded,
                Err(e) => panic!("Could not decode from hex: {}", e),
            };
            
            // Iterate through all the XOR keys and score the resulting stream.
            for key in 0..=255 {
    
                // Try a key from the set of all one byte keys.
                let decipher_attempt = xor_by_single_byte(&raw_bytes, key);
    
                // Calculate a score for each key attempt based on how close to
                // english text it is.
                let score = calculate_score(&decipher_attempt);

                if score < lowest_score {
                    lowest_score = score;
                    cipher_key = key;
                    decrypted_string = decipher_attempt;
                }
            }
        }
        
        // Print the best scoring string from the input data.
        println!("key: {}", cipher_key as char);
    
        println!("{:?}", decrypted_string);
    
        if let Ok(string_from_bytes) = String::from_utf8(decrypted_string) {
            println!("Deciphered String: {}", string_from_bytes);
        } else {
            println!("String conversion failed");
        }
}

fn main() {

    let reader = match read_file_lines("encrypted.txt") {
        Ok(reader) => reader,
        Err(e) => {
            eprint!("Error creating BufReader: {}", e);
            std::process::exit(1);
        }
    };

    detect_single_byte_xor(&reader);
}