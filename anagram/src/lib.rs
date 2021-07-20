use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter_map(|candidate| {
            if candidate.eq_ignore_ascii_case(word) {
                return None;
            }
            let mut temp = String::from(word);
            for c in candidate.chars() {
                if let Some(pos) = temp.chars().position(|x| x.eq_ignore_ascii_case(&c)) {
                    temp.replace_range(pos..pos + c.len_utf8(), "");
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
