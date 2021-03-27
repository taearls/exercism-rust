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

fn get_quantity_from_char(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("c is not numeric: {}", c),
    }
}