use std::fs::{self, read};
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

fn main() {
    let encoded_message = match read_and_decode_file("base64.txt") {
        Ok(contents) => contents,
        Err(e) => {
            eprint!("Error reading and decoding file: {}", e);
            std::process::exit(1);
        }
    };

}
