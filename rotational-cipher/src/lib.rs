const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            match ALPHABET
                .iter()
                .position(|&x| x.to_ascii_lowercase() == c.to_ascii_lowercase())
            {
                Some(x) => {
                    let mut index = ((x as i8) + key) % 26;

                    // if index is negative because of a negative key, keep adding 26 until it's positive.
                    while index < 0 {
                        index += 26;
                    }

                    if c.is_ascii_uppercase() {
                        ALPHABET[index as usize].to_ascii_uppercase()
                    } else {
                        ALPHABET[index as usize]
                    }
                }
                None => c,
            }
        })
        .collect()
}
