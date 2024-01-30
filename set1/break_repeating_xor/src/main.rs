use std::fs;
use phf::phf_map;
use std::cmp::Ordering;
use base64::{engine::general_purpose::STANDARD, Engine as _};

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

pub fn xor_by_single_byte(stream: &[u8], byte: u8) -> Vec<u8>{
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

pub fn transpose(encoded_message: &[u8]) -> Vec<Vec<u8>> {
    return (0..29).map(|i| encoded_message.iter().skip(i).step_by(29).cloned().collect()).collect();
}

pub fn break_single_char_xor(message_chunk: &[u8]) -> u8 {
    // Estblish variables to hold best scoring key and string.
    let mut cipher_key: u8 = 0;
    let mut highest_score: f32 = 0.0;

    // Iterate through all the XOR keys and score the resulting stream.
    for key in 0..=255 {

        // Try a key from the set of all one byte keys.
        let decipher_attempt = xor_by_single_byte(message_chunk, key);

        // Establish a variable to hold the score.
        let mut total_score = 0.0;

        for byte in &decipher_attempt {
            total_score += score_byte(*byte);
        }

        //Compare the score of each window with the best score so far.
        if total_score > highest_score {
            cipher_key = key;
            highest_score = total_score;
        }
    }

    // Print the best scoring string from the input data.
    println!("key: {}", cipher_key as char);
    return cipher_key;
}

pub fn break_repeating_xor(encoded_message: &[u8], keysize: &u8) {
    let chunked_message = transpose(encoded_message);

    for i in 0..*keysize {
        let key = break_single_char_xor(&chunked_message[i as usize]);
        println!("{}", key);
    }

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

    let encryption_key = break_repeating_xor(&encoded_message, &keysize);
    
}
