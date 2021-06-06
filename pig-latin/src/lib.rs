use lazy_static::lazy_static;
use regex::Regex;

pub fn translate(input: &str) -> String {
    input.split_whitespace().into_iter().map(|word| {
        let mut temp_str = word.to_string();
        match word.chars().next() {
            Some(c) if matches_vowels(&c.to_string()) => temp_str.push_str("ay"),
            Some(c) => {
                if let Some(found) = matches_consonant(word) {
                    temp_str.replace_range(..found[0].len(), "");
                    temp_str.push_str(&found[0]);
                } else if !chars_to_be_ignored(&word[0..2]) {
                    temp_str.remove(0);
                    temp_str.push(c);
                } 
                temp_str.push_str("ay");
            },
            None => (),
        }
        temp_str
    }).collect::<Vec<_>>().join(" ")
}

fn matches_consonant(input: &str) -> Option<regex::Captures<'_>> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(?i)(s?ch|s?qu|thr?|rh)").unwrap();
    }
    RE.captures(input)
}

fn matches_vowels(input: &str) -> bool {
    lazy_static! {
        static ref VOWEL_REGEX: Regex = Regex::new("(?i)(a|e|i|o|u)").unwrap();
    }
    VOWEL_REGEX.is_match(input)
}

fn chars_to_be_ignored(input: &str) -> bool {
    lazy_static! {
        static ref CHARS_TO_IGNORE: Regex = Regex::new("(yt|xr)").unwrap();
    }
    CHARS_TO_IGNORE.is_match(input)
}