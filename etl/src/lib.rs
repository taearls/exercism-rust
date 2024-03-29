use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result: BTreeMap<char, i32> = BTreeMap::new();
    for (score, vec_chars) in h {
        for vec_char in vec_chars {
            result.insert(vec_char.to_ascii_lowercase(), *score);
        }
    }
    result
}
