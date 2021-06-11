pub fn encrypt(input: &str) -> String {
    let (rows, cols) = get_str_rect(input);

    let filtered_str = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();

    // create vec with empty strings of correct length
    let mut str_vec = (0..cols)
        .map(|_| String::with_capacity(cols))
        .collect::<Vec<_>>();

    // create padded string so we account for extra spaces
    let padded_str = format!("{:width$}", filtered_str, width = rows * cols);

    for (i, c) in padded_str.chars().enumerate() {
        str_vec[i % cols].push(c);
    }
    str_vec.join(" ")
}

fn get_str_rect(input: &str) -> (usize, usize) {
    let trimmed_str: String = input.split_whitespace().collect();
    let sqrt = (trimmed_str.len() as f32).sqrt();
    (sqrt.floor() as usize, sqrt.ceil() as usize)
}
