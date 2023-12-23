use phf::phf_map;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Frequency of letter appearance in concise oxford dictionary(9th edition, 1995).
static EXPECTED_FREQUENCY: phf::Map<u8, f32> = phf_map! {
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

pub fn xor_by_single_byte(stream: &Vec<u8>, byte: u8) -> Vec<u8>{
    return stream.iter().map(|b| b ^ byte).collect();
}

pub fn is_alphabetic(byte: u8) -> bool {
    (byte >= 0x41 && byte <= 0x5A) || (byte >= 0x61 && byte <= 0x7A)
}

pub fn score_byte(byte: u8) -> f32 {
    if is_alphabetic(byte) {
        return match EXPECTED_FREQUENCY.get(&byte) {
            Some(frequency) => *frequency,
            None => 0.0,
        };
    } else {
        return 0.0;
    }
}

fn main() -> Result<(), std::io::Error> {
    // Attempt to open file and propogate error if not (?).
    let file = File::open("encrypted.txt")?;

    // Feed the contents of the file into a buffer reader.
    let reader = BufReader::new(file);

    // Estblish variables to hold best scoring key and string.
    let mut cipher_key: u8 = 0;
    let mut highest_score: f32 = 0.0;
    let mut decrypted_string: Vec<u8> = Vec::new();

    // Iterate over the lines of the file.
    for line in reader.lines() {
        // Convert from hex String to Vec<u8>.
        let raw_bytes = match hex::decode(line?) {
            Ok(decoded) => decoded,
            Err(e) => panic!("Could not decode from hex: {}", e),
        };
        
        // Iterate through all the XOR keys and score the resulting stream.
        for key in 0..=255 {

            // Try a key from the set of all one byte keys.
            let decipher_attempt = xor_by_single_byte(&raw_bytes, key);

            // Establish a variable to hold the score.
            let mut total_score = 0.0;

            for byte in &decipher_attempt {
                total_score += score_byte(*byte);
            }

            //Compare the score of each window with the best score so far.
            if total_score > highest_score {
                cipher_key = key;
                highest_score = total_score;
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

    Ok(())

}