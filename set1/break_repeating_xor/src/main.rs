use std::fs;
use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn hamming_distance(string1 : &[u8], string2 : &[u8]) -> u32 {
    if string1.len() != string2.len() {
        println!("Hamming distance input has strings of uneven length!");
    }
    
    // Create a vector to hold the XOR'd product of the two strings.
    let x: Vec<u8> = string1.iter()
                    .zip(string2.iter())
                    .map(|(&a, &b)| a ^ b)
                    .collect();

    let mut count = 0;

    // Iterate over the bytes of the XOR'd product and count the set bits.
    for byte in x {
        count += u8::count_ones(byte);
    }

    return count;
}

fn main() {

    // Test to see whether hamming_distance passes the test set in the challenge.
    assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37);

    // Read the contents of the file into a Vec<u8>.
    let mut b64 = match fs::read("base64.txt") {
        Ok(read) => read,
        Err(e) => panic!("Could not read file: {}", e),
    };

    // remove the newline characters from the b64 input.
    b64.retain(|byte| *byte != b'\n');

    // Decode the text from base64.
    let encoded = match STANDARD.decode(b64) {
        Ok(result) => result,
        Err(e) => panic!("Could not decode from base64: {e}"),
    };

    // Create a Vector of tuples to store the keylengths and hamming distances.
    let mut key_scores: Vec<(u8, u32)> = Vec::new();

    for keysize in 2..=40 {
        let first_chunk: &[u8] = &encoded[0..keysize];
        let second_chunk: &[u8] = &encoded[keysize..keysize + keysize];

        // Insert each keylength and hamming distances to the Vector.
        key_scores.push((keysize.try_into().unwrap(), hamming_distance(first_chunk, second_chunk)));
    }

    // Sort by hamming distance values.
    key_scores.sort_by(|a, b| a.1.cmp(&b.1));

    println!("{:?}", key_scores);

    
}
