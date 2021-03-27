pub fn encode(source: &str) -> String {
    let mut encoded_str = String::with_capacity(source.len());
    let mut temp_quantity: u8 = 1;
    let mut temp_letter = String::with_capacity(1);
    for c in source.chars() {
        if temp_letter.starts_with(c) {
            temp_quantity += 1;
        } else if temp_letter.is_empty() {
            temp_letter.push(c);
        } else {
            if temp_quantity > 1 {
                encoded_str.push_str(&temp_quantity.to_string());
            }
            encoded_str.push_str(&temp_letter);

            // reset temp vars
            temp_letter.clear();
            temp_letter.push(c);
            temp_quantity = 1;
        }
    }
    // have to process for length of chars + 1 to account for initial allocation of first char
    if temp_quantity > 1 {
        encoded_str.push_str(&temp_quantity.to_string());
    }
    encoded_str.push_str(&temp_letter);

    encoded_str
}

pub fn decode(source: &str) -> String {
    let mut decoded_str = String::new();
    let mut temp_quantity_str = String::new();
    for c in source.chars() {
        if c.is_numeric() {
            temp_quantity_str.push(c);
        } else {
            let temp_quantity: u8 = temp_quantity_str.parse().unwrap_or(1);
            for _ in 0..temp_quantity {
                decoded_str.push(c);
            }
            temp_quantity_str.clear();
        }
    }
    decoded_str
}
