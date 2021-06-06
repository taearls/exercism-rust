pub fn encrypt(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .enumerate()
        .fold(String::new(), |acc, (i, c)| {
            // every 5 chars add a space
            if i != 0 && i % 4 == 0 {
                format!("{} {}", acc, c)
            } else {
                format!("{}{}", acc, c)
            }
        })
}
