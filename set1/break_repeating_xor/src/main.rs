use std::fs;
use std::cmp::Ordering;
use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn read_and_decode_file(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    // Read the contents of the file into a Vec<u8>.
    let mut b64 = match fs::read(filename) {
        Ok(read) => read,
        Err(e) => panic!("Could not read file: {}", e),
    };

    // remove the newline characters from the b64 input.
    b64.retain(|byte| *byte != b'\n');

    // Decode the text from base64.
    let encoded = match STANDARD.decode(b64) {
        Ok(result) => result,
        Err(e) => panic!("Could not decode from base64: {}", e),
    };

    return Ok(encoded);
}

pub fn hamming_distance(string1 : &[u8], string2 : &[u8]) -> f32 {
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

    return count as f32;
}

pub fn find_likeliest_keysize(encoded_message: &[u8]) -> u8 {
    // Create a Vector of tuples to store the keylengths and hamming distances.
    let mut key_scores: Vec<(u8, f32)> = Vec::new();

    for keysize in 2..=40 {
        // Take the first four chunks of the message.
        let first_chunk: &[u8] = &encoded_message[0..keysize];
        let second_chunk: &[u8] = &encoded_message[keysize..2*keysize];
        let third_chunk: &[u8] = &encoded_message[2*keysize..3*keysize];
        let fourth_chunk: &[u8] = &encoded_message[3*keysize..4*keysize];

        // Calculate the distances between the chunks.
        let dist_1_2 = hamming_distance(&first_chunk, &second_chunk);
        let dist_1_3 = hamming_distance(&first_chunk, &third_chunk);
        let dist_1_4 = hamming_distance(&first_chunk, &fourth_chunk);
        let dist_2_3 = hamming_distance(&second_chunk, &third_chunk);
        let dist_2_4 = hamming_distance(&second_chunk, &fourth_chunk);
        let dist_3_4 = hamming_distance(&third_chunk, &fourth_chunk);

        let normalized_average_dist: f32 = (dist_1_2 + dist_1_3 + dist_1_4 + dist_2_3 + dist_2_4 + dist_3_4)
            as f32
            / (6.0 * keysize as f32);

        // Insert each keylength and hamming distance to the Vector.
        key_scores.push((keysize.try_into().unwrap(), normalized_average_dist));
    }

    // Sort by hamming distance values. This is probably q
    key_scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));

    // Return keyscore with the lowest hamming distance.
    return key_scores[0].0;
}

fn main() {

    // Test to see whether hamming_distance passes the test set in the challenge.
    assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37.0);

    let encoded_message = match read_and_decode_file("base64.txt") {
        Ok(contents) => contents,
        Err(e) => {
            eprint!("Error reading and decoding file: {}", e);
            std::process::exit(1);
        }
    };

    let keysize = find_likeliest_keysize(&encoded_message);

    println!("{}", keysize);
    
}
