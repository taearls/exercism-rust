const ALPHABET: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

/// 'Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
    .chars()
    .filter(|c| c.is_alphanumeric())
    .map(|c| {
        if c.is_alphabetic() {
            let index = ALPHABET
                .iter()
                .position(|p| &c.to_lowercase().next().unwrap() == p)
                .unwrap();
            let reversed_index = ALPHABET.len() - 1 - index;
            ALPHABET[reversed_index]
        } else {
            c
        }
    })
    .enumerate()
    .fold(String::new(), |acc, (i, c)| {
        // every 5 chars add a space
        if i != 0 && i % 5 == 0 {
            format!("{} {}", acc, c)
        } else {
            format!("{}{}", acc, c)
        }
    })
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    unimplemented!("Decoding of {:?} in Atbash cipher.", cipher);
}
