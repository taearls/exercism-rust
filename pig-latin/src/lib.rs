use lazy_static::lazy_static;
use regex::Regex;

pub fn translate(input: &str) -> String {
    let mut result = String::from(input);
    match input.chars().next() {
        Some(c) if matches_vowels(&c.to_string()) => result.push_str("ay"),
        Some(c) => {
            if let Some(found) = matches_consonant(input) {
                result.replace_range(..found[0].len(), "");
                result.push_str(&found[0]);
            } else if !chars_to_be_ignored(&input[0..2]) {
                result.remove(0);
                result.push(c);
            } 
            result.push_str("ay");
        },
        None => (),
    }
    result
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