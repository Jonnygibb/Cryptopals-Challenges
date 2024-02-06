use std::{fs, collections::HashMap};
use phf::phf_map;
use std::cmp::Ordering;
use base64::{engine::general_purpose::STANDARD, Engine as _};

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
    // Find the number of occurances that each letter or punctuation
    // has in a piece of text.
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


pub fn xor_by_single_byte(stream: &[u8], byte: u8) -> Vec<u8>{
    return stream.iter().map(|b| b ^ byte).collect();
}

pub fn is_alphabetic(byte: u8) -> bool {
    (byte >= 0x41 && byte <= 0x5A) || (byte >= 0x61 && byte <= 0x7A)
}

pub fn is_control(u: u8) -> bool {
    u < 0x20 || u == 0x7F
}

pub fn vec_to_string(vec: &Vec<u8>) -> String {
    let s = String::from_utf8_lossy(vec);
    return s.to_string();
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

pub fn transpose(encoded_message: &[u8], keysize: &u8) -> Vec<Vec<u8>> {
    let block = *keysize as usize;
    return (0..block).map(|i| encoded_message.iter().skip(i.into()).step_by(block).cloned().collect()).collect();
}

pub fn break_single_char_xor(message_chunk: &[u8]) -> u8 {
    // Estblish variables to hold the lowest scoring key attempt.
    let mut cipher_key: u8 = 0;
    let mut lowest_score: f64 = std::f64::MAX;

    // Iterate through all the XOR keys and score the resulting stream.
    for key in 0..=255 {

        // Try a key from the set of all one byte keys.
        let decipher_attempt = xor_by_single_byte(message_chunk, key);

        // Calculate a score for each key attempt based on how close to
        // english text it is.
        let score = calculate_score(&decipher_attempt);

        if score < lowest_score {
            lowest_score = score;
            cipher_key = key;
        }
    }
    return cipher_key;
}

pub fn break_repeating_xor(encoded_message: &[u8], keysize: &u8) -> Vec<u8> {
    let chunked_message = transpose(encoded_message, keysize);
    let mut encryption_key: Vec<u8> = Vec::<u8>::new();

    for i in 0..*keysize {
        let key = break_single_char_xor(&chunked_message[i as usize]);
        encryption_key.push(key);
    }

    return encryption_key;
}

pub fn repeating_xor(message: &[u8], key: &[u8]) -> Vec<u8> {
    
    // Iterate over the message and apply the characters of key cyclicalally.
    let encrypted = message.iter()
                           .zip(key.iter().cycle())
                           .map(|(&a, &b)| a ^ b)
                           .collect();
    return encrypted;
}

fn main() {

    // Test to see whether hamming_distance passes the test set in the challenge.
    assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37.0);

    // Collect the contents of the file to decode.
    let encoded_message = match read_and_decode_file("base64.txt") {
        Ok(contents) => contents,
        Err(e) => {
            eprint!("Error reading and decoding file: {}", e);
            std::process::exit(1);
        }
    };

    let keysize = find_likeliest_keysize(&encoded_message);

    let encryption_key = &break_repeating_xor(&encoded_message, &keysize);

    println!("Encryption key:\n{}", vec_to_string(encryption_key));
    
    let decrypted_file = &repeating_xor(&encoded_message, &encryption_key);

    println!("Decrypted text:\n{}", vec_to_string(decrypted_file));
}
