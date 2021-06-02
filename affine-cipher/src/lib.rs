#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

// E(x) = (ax + b) mod m
// where x is the letter's index from 0 - length of alphabet - 1
// m is the length of the alphabet. For the roman alphabet m == 26.
// and a and b make the key
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a) {
        Err(AffineCipherError::NotCoprime(a))
    } else {
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
}

// D(y) = a^-1(y - b) mod m
// where y is the numeric value of an encrypted letter, ie. y = E(x)
// it is important to note that a^-1 is the modular multiplicative inverse of a mod m
// the modular multiplicative inverse of a only exists if a and m are coprime.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let mut str = String::with_capacity(ciphertext.len());
    let mmi = find_mmi(a)?;
    
    for c in ciphertext.chars() {
        if !c.is_alphanumeric() { continue; }
        if c.is_numeric() {
            str.push(c);
        } else if c.is_alphabetic() {
            let mut new_char_val = (mmi * (letter_index(c) - b)) % 26;
            // adding 26 here handles case where result of new_char_val is negative
            new_char_val = (new_char_val + 26) % 26;

            let new_char = ((new_char_val as u8) + 97) as char;
            str.push(new_char as char);
        }
    }
    Ok(str)
}

fn letter_index(c: char) -> i32 {
    // 'a' has a value of 97
    c.to_ascii_lowercase() as i32 - 97
}

fn is_coprime(a: i32) -> bool {
    // a must be coprime with m (in this case, 26)
    a % 2 != 0 && a % 13 != 0
}

// 1 = (a * x) mod m, where x is modular multiplicative inverse
fn find_mmi(a: i32) -> Result<i32, AffineCipherError> {
    for i in 2..=25 {
        if (a * i) % 26 == 1 {
            return Ok(i)
        }
    }
    Err(AffineCipherError::NotCoprime(a))
}
