use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let alphabet_len = 26;
    let mut hashset: HashSet<char> = HashSet::with_capacity(alphabet_len);

    for c in sentence.to_ascii_lowercase().chars() {
        if !c.is_ascii_lowercase() {
            continue;
        }
        hashset.insert(c);
    }
    hashset.len() == alphabet_len
}
