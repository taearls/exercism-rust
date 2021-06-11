pub fn encrypt(input: &str) -> String {
    let (rows, row_length) = get_chunk_length(input);
    let mut result = String::with_capacity(rows * row_length);

    let filtered_str = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .collect::<String>();

    let padded_str = format!("{:width$}", filtered_str, width = rows * row_length);

    let mut str_vec: Vec<&str> = Vec::new();
    for row in 0..rows {
        let index = row * row_length;
        str_vec.push(padded_str.get(index..index + row_length).unwrap())
    }

    for col in 0..row_length {
        let mut index = col;
        while index < padded_str.len() {
            result.push_str(padded_str.get(index..index + 1).unwrap());
            index += row_length;
        }
        if col < row_length - 1 {
            result.push(' ');
        }
    }
    result
}
fn get_chunk_length(input: &str) -> (usize, usize) {
    let trimmed_str: String = input.split_whitespace().collect();
    let sqrt = (trimmed_str.len() as f32).sqrt();
    (sqrt.floor() as usize, sqrt.ceil() as usize)
}
