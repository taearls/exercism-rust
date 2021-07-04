use rand::random;

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const RAND_KEY_LEN: usize = 100;

fn shift_str<F: Fn(usize, usize) -> usize>(key: &str, s: &str, op: F) -> Option<String> {
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
                    let key_index = i % key.len();

                    let key_char = key.get(key_index..key_index + 1).unwrap();
                    let key_char_pos = ALPHABET
                        .iter()
                        .position(|&x| x == key_char.chars().nth(0).unwrap())
                        .unwrap();
                    let old_alphabet_pos = ALPHABET.iter().position(|&x| x == c).unwrap();

                    let shifted_char_alphabet_index =
                        op(old_alphabet_pos + ALPHABET.len(), key_char_pos) % ALPHABET.len();
                    let shifted_char = ALPHABET
                        .get(shifted_char_alphabet_index..shifted_char_alphabet_index + 1)
                        .unwrap();

                    (i, shifted_char[0])
                })
                .fold(String::new(), |acc, (_i, c)| {
                    let mut buf = [0; 2];
                    acc + c.encode_utf8(&mut buf)
                }),
        )
    }
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    shift_str(key, s, |x, y| x + y)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    shift_str(key, s, |x, y| x - y)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = (0..RAND_KEY_LEN)
        .map(|_| {
            let random_index: usize = random::<usize>() % ALPHABET.len();
            ALPHABET.get(random_index).unwrap()
        })
        .fold(String::new(), |acc, c| {
            let mut buf = [0; 2];
            acc + c.encode_utf8(&mut buf)
        });
    (key.clone(), encode(&key, &s).unwrap())
}
