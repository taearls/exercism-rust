#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let mut str = String::with_capacity(plaintext.len());
    let mut alphanumeric_char_count: usize = 0;

    for c in plaintext.chars() {
        if !c.is_ascii_alphanumeric() { continue; }
        
        // every 5 chars prepend a space to result str before adding new_char
        if alphanumeric_char_count > 0 && alphanumeric_char_count % 5 == 0 {
            str.push(' ');
        }
        if c.is_alphabetic() {
            let new_char = ((a * letter_index(c) + b) % 26) as u8 + 97;
            str.push(new_char as char);
        } else if c.is_numeric() {
            let new_char = c;
            str.push(new_char);
        }
        alphanumeric_char_count += 1;
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