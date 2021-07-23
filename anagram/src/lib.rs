use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lowercased_word = word.to_lowercase();
    let sorted_word = sort_word(&lowercased_word);
    possible_anagrams
        .iter()
        .filter_map(|candidate| {
            let lowercased_candidate = candidate.to_lowercase();
            if lowercased_candidate != lowercased_word
                && sort_word(&lowercased_candidate) == sorted_word
            {
                Some(*candidate)
            } else {
                None
            }
        })
        .collect::<HashSet<&'a str>>()
}

fn sort_word(word: &str) -> Vec<char> {
    let mut v: Vec<char> = word.chars().collect();
    v.sort_unstable();
    v
}
