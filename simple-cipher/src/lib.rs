use rand::random;

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const RAND_KEY_LEN: usize = 100;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty()
        || s.is_empty()
        || s.chars().filter(|c| !c.is_ascii_lowercase()).count() > 0
        || key.chars().filter(|c| !c.is_ascii_lowercase()).count() > 0
    {
        None
    } else {
        Some(
            s.char_indices()
                .map(|(i, c)| {
                    let old_char_pos = ALPHABET.iter().position(|&x| x == c).unwrap();
                    let key_index = i % key.len();
                    let current_key = key.get(key_index..key_index + 1).unwrap();
                    let current_key_pos = ALPHABET
                        .iter()
                        .position(|&x| x.to_string() == current_key)
                        .unwrap();
                    let new_index = (old_char_pos + current_key_pos) % ALPHABET.len();
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
    if key.is_empty()
        || s.is_empty()
        || s.chars().filter(|c| !c.is_ascii_lowercase()).count() > 0
        || key.chars().filter(|c| !c.is_ascii_lowercase()).count() > 0
    {
        None
    } else {
        Some(
            s.char_indices()
                .map(|(i, c)| {
                    let old_char_pos = ALPHABET.iter().position(|&x| x == c).unwrap();
                    let key_index = i % key.len();
                    let current_key = key.get(key_index..key_index + 1).unwrap();
                    let current_key_pos = ALPHABET
                        .iter()
                        .position(|x| x.to_string() == current_key)
                        .unwrap();
                    let new_index =
                        (old_char_pos + ALPHABET.len() - current_key_pos) % ALPHABET.len();
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

pub fn encode_random(s: &str) -> (String, String) {
    let key = (0..RAND_KEY_LEN).map(|_| {
        let random_index: usize = random::<usize>() % ALPHABET.len();
        ALPHABET.get(random_index).unwrap()
    }).fold(String::new(), |acc, c| {
        let mut buf = [0; 2];
        acc + c.encode_utf8(&mut buf)
    });
    (key.clone(), encode(&key, &s).unwrap())
}
