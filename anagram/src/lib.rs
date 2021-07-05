use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    possible_anagrams.iter().filter_map(|candidate| {
        if word == *candidate {
            Some(*candidate)
        } else {
            None
        }
    })
    .collect::<HashSet<&'a str>>()
}
