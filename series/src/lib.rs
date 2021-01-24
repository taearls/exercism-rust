pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    
    if len <= digits.len() {
        let mut start_index = 0;
        loop {
            if start_index + len > digits.len() {
                break;
            }
            vec.push(
                digits.get(
                        start_index..(start_index + len)
                    )
                    .unwrap()
                    .to_string()
            );

            start_index += 1;
        }
    }

    vec
}
