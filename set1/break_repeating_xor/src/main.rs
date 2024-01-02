

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
    let KEYSIZE = 0;

    // Test to see whether hamming_distance passes the test set in the challenge.
    let string1 = b"this is a test";
    let string2 = b"wokka wokka!!!";

    let i = hamming_distance(string1, string2);

    println!("Test value is: {}", i);
}
