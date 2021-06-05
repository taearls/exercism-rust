pub fn translate(input: &str) -> String {
    let mut result = String::from(input);
    match input.chars().nth(0) {
        Some('A' | 'a' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U') => result.push_str("ay"),
        Some(c) => {
            result.remove(0);
            result.push_str(&format!("{}ay", c));
        },
        None => (),
    }
    result
}
