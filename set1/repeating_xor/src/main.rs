use hex;

pub fn repeating_xor( message: &[u8],  key: &[u8]) -> Vec<u8> {
    
    // Iterate over the message and apply the characters of key cyclicalally.
    let encrypted = message.iter()
                           .zip(key.iter().cycle())
                           .map(|(&a, &b)| a ^ b)
                           .collect();
    return encrypted;
}

fn main() {
    // Message and key from challenge
    let message = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";

    // Apply the repeating xor to the message using the specified key.
    let encrypt_attempt = repeating_xor(message, key);

    // Encode the encrypted bytes into hex.
    let encrypted_hex = hex::encode(encrypt_attempt);

    // Print out the encrypted message.
    println!("Encrypted message: {}", encrypted_hex);

    // Assert the returned string matches the test case in the challenge.
    assert_eq!(
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c\
        2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b\
        2027630c692b20283165286326302e27282f",
        encrypted_hex
        );
}
