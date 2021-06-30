const ALPHABET: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

pub fn rotate(input: &str, key: i8) -> String {
    input
        .to_ascii_lowercase()
        .chars()
        .map(|c| {
            let old_val = ALPHABET.iter().position(|&x| x == c).unwrap();
            let new_val = (old_val + (key as usize)) % 26;
            return ALPHABET[new_val];
        })
        .collect()
}
