use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    possible_anagrams.iter().filter_map(|candidate| {
        let mut temp = String::from(word);
        for c in candidate.chars() {
            if let Some(pos) = temp.chars().position(|x| x == c) {
                temp.replace_range(pos..pos + 1, "");
            } else {
                return None;
            }
        }
        
        if !temp.is_empty() {
            None
        } else {
            Some(*candidate)
        }
    })
    .collect::<HashSet<&'a str>>()
}
