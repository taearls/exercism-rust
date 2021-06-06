const ALPHABET: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

/// 'Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
    .chars()
    .filter(|c| c.is_alphabetic())
    .map(|c| {
        let index = ALPHABET.iter().position(|p| &c.to_lowercase().next().unwrap() == p).unwrap();
        let reversed_index = ALPHABET.len() - 1 - index;
        ALPHABET[reversed_index]
    })
    .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    unimplemented!("Decoding of {:?} in Atbash cipher.", cipher);
}
