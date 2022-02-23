use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut result: HashMap<String, u32> = HashMap::new();

    for word in words.split_whitespace() {
        let mut base: u32 = 0;
        if let Some(&val) = result.get(word) {
            base = val;
        }
        result.insert(word.to_string(), base + 1);
    }
    result
}
