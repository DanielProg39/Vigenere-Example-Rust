fn encrypt(plaintext: &str, key: &str) -> String {
    // Store the length of the key as a u32
    let m = key.len() as u32;
    // Declare a new empty string to store the ciphertext
    let mut ciphertext = String::new();
    // Keep track of current position of key
    let mut key_pos = 0;
    // Iterate through the plaintext, getting the character of each element
    for character in plaintext.chars() {
        // Check if current character is a space
        if !character.is_whitespace() {
            // Shift the character by the corresponding character in the key, using the modulus operator
            // to wrap around to the start of the key if we've exceeded its length
            let shifted = (character as u32 + key.chars().nth(key_pos % m as usize).unwrap() as u32) % 26;
            // Add the shifted character to the ciphertext, converting it back to a character
            // by adding the ASCII value of 'A'
            ciphertext.push((shifted as u8 + b'A') as char);
            key_pos += 1;
        } else {
            // if it is a space, just append it to the ciphertext
            ciphertext.push(character);
        }
    }
    // Return the completed ciphertext
    ciphertext
}

// main() is used purely to test the code, just hardcode plaintext and key values below
// Note that this program works properly only with capital letters of English alphabet and spaces,
// so consider enabling Caps Lock before changing anything)
fn main() {
    let plaintext = "HELLO WORLD";
    let key = "KEY";
    let ciphertext = encrypt(plaintext, key);
    println!("Plaintext: {}", plaintext);
    println!("Key: {}", key);
    println!("Ciphertext: {}", ciphertext);
}
