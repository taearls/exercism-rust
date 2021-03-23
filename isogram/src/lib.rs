use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut result = true;
    let mut char_count_hm: HashMap<char, u8> = HashMap::new();
    // lowercase and filter out ' ' and '-' chars
    let lowercased = candidate
        .replace(|ch| ch == ' ' || ch == '-', "")
        .to_lowercase();
    for letter in lowercased.chars() {
        *char_count_hm.entry(letter).or_insert(0) += 1;
        let count = *char_count_hm.get(&letter).unwrap();
        println!("{} count: {}", letter, count);
        if count > 1 {
            result = false;
            break;
        }
    }
    result
}
