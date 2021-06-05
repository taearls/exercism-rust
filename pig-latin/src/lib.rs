use lazy_static::lazy_static;
use regex::Regex;



pub fn translate(input: &str) -> String {
    let mut result = String::from(input);
    match input.chars().nth(0) {
        Some('A' | 'a' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U') => result.push_str("ay"),
        Some(c) => {
            if let Some(found) = matches_consonant(input) {
                result.replace_range(..found[0].len(), "");
                result.push_str(&found[0]);
            } else {
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
        static ref RE: Regex = Regex::new("s?ch|s?qu|thr?|rh").unwrap();
    }
    RE.captures(input)
}