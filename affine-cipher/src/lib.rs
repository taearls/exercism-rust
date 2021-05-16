#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let mut str = String::with_capacity(plaintext.len());
    for c in plaintext.chars() {
        let new_char = ((a * letter_index(c) + b) % 26) as u8 + 97;
        str.push(new_char as char);
    }
    Ok(str)
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    unimplemented!("Decode {} with the key ({}, {})", ciphertext, a, b);
}

fn letter_index(c: char) -> i32 {
    // 'a' has a value of 97
    c.to_ascii_lowercase() as i32 - 97
}