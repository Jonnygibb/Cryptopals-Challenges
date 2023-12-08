use hex;
use phf::phf_map;

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

fn main() {
    let encrypted_hex: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let encrypted_bytes = match hex::decode(&encrypted_hex) {
        Ok(byte_stream) => byte_stream,
        Err(e) => panic!("Error thrown: {}", e),
    };

    let mut cipher_key: u8 = 0;
    let mut highest_score: f32 = 0.0;
    let mut decrypted_bytes: Vec<u8> = Vec::new();

    for key in 0..=255 {
        let decipher_attempt = xor_by_single_byte(&encrypted_bytes, key);

        let mut total_score = 0.0;

        for byte in &decipher_attempt {
            total_score += score_byte(*byte);
        }

        if total_score > highest_score {
            cipher_key = key;
            highest_score = total_score;
            decrypted_bytes = decipher_attempt;
        }
    }

    println!("key: {}", cipher_key as char);

    if let Ok(string_from_bytes) = String::from_utf8(decrypted_bytes) {
        println!("Deciphered String: {}", string_from_bytes);
    } else {
        println!("String conversion failed");
    }
}
