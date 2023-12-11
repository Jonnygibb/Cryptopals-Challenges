use std::fs;

fn main() {
    // Create path to the encrypted strings
    let file_path = "encrypted.txt";

    // Read the hex contents of the file.
    let contents: Vec<u8> = fs::read(file_path).expect("Should have been able to read the file");

    // Split the strings by the newline character.
    let strings: Vec<_> = contents.split(|&c| c == b'\n').collect();
    
    println!("{}", strings.len());
    
    for string in strings {
        let bytes = match hex::decode(&string) {
        Ok(byte_stream) => byte_stream,
        Err(e) => panic!("Error thrown: {}", e),
        };
        println!("{:?}", bytes);
    }
    
}
