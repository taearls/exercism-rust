pub fn encode(source: &str) -> String {
    let mut encoded_str = String::with_capacity(source.len());
    // let mut temp_quantity: u8 = 1;
    // let mut temp_letter: char;
    
    // for s in source.chars() {
    //     if s.is_numeric() {
    //         temp_quantity = s as u8;
    //     } else if s.is_alphabetic() {
    //         for _ in 0..temp_quantity {
    //             encoded_str.push(s);
    //         }
    //         temp_quantity = 1;
    //     }
    // }

    encoded_str
}

pub fn decode(source: &str) -> String {
    let mut decoded_str = String::new();
    let mut temp_quantity: u8 = 1;
    for s in source.chars() {
        if s.is_numeric() {
            temp_quantity = s as u8;
        } else {
            println!("temp_quantity: {}", temp_quantity);
            for _ in 0..temp_quantity {
                decoded_str.push(s);
            }
            temp_quantity = 1;
        }
    }
    decoded_str
}
