pub fn encrypt(input: &str) -> String {
    let chunk_length = get_chunk_length(input);
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .enumerate()
        .fold(String::new(), |acc, (i, c)| {
            if i != 0 && i % chunk_length == 0 {
                format!("{} {}", acc, c)
            } else {
                format!("{}{}", acc, c)
            }
        })
}
fn get_chunk_length(input: &str) -> usize {
    let trimmed_input: String = input.split_whitespace().collect();
    (trimmed_input.len() as f32).sqrt().ceil() as usize
}