const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.len() != s.len()
        || s.chars().filter(|c| !c.is_ascii_lowercase()).count() > 0
        || key.chars().filter(|c| !c.is_ascii_lowercase()).count() > 0
    {
        None
    } else {
        Some(
            s.char_indices()
                .map(|(i, c)| {
                    let alphabet_pos = ALPHABET.iter().position(|&x| x == c).unwrap();
                    let key_at_pos = key.get(i..i + 1).unwrap();
                    let key_pos = ALPHABET
                        .iter()
                        .position(|&x| x.to_string() == key_at_pos)
                        .unwrap();
                    let new_index = (alphabet_pos + key_pos) % 26;
                    let new_char = ALPHABET.get(new_index..new_index + 1).unwrap();
                    (i, new_char[0])
                })
                .fold(String::new(), |acc, (_i, c)| {
                    let mut buf = [0; 2];
                    acc + c.encode_utf8(&mut buf)
                }),
        )
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    unimplemented!("Use {} to decode {} using shift cipher", key, s)
}

pub fn encode_random(s: &str) -> (String, String) {
    unimplemented!(
        "Generate random key with only a-z chars and encode {}. Return tuple (key, encoded s)",
        s
    )
}
