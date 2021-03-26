pub fn encode(n: u64) -> String {
    let num_str = n.to_string();
    match num_str.len() {
        1 => handle_ones(&num_str),
        2 => handle_tens(&num_str),
        3 => handle_hundreds(&num_str),
        4..=6 => handle_thousands(&num_str),
        7..=9 => handle_millions(&num_str),
        10..=12 => handle_billions(&num_str),
        13..=15 => handle_trillions(&num_str),
        16..=18 => handle_quadrillions(&num_str),
        19..=20 => handle_quintillions(&num_str),
        _ => panic!("invalid num_str: {}", &num_str),
    }
}

fn handle_ones(num_str: &str) -> String {
    match num_str.get(0..1) {
        Some("0") => "zero".to_string(),
        Some("1") => "one".to_string(),
        Some("2") => "two".to_string(),
        Some("3") => "three".to_string(),
        Some("4") => "four".to_string(),
        Some("5") => "five".to_string(),
        Some("6") => "six".to_string(),
        Some("7") => "seven".to_string(),
        Some("8") => "eight".to_string(),
        Some("9") => "nine".to_string(),
        _ => panic!("invalid num_str: {}", num_str),
    }
}

fn handle_tens(num_str: &str) -> String {
    let tens_name = match num_str.get(0..1) {
        Some("0") => "",
        Some("1") => match num_str.get(1..2) {
            Some("0") => "ten",
            Some("1") => "eleven",
            Some("2") => "twelve",
            Some("3") => "thirteen",
            Some("4") => "fourteen",
            Some("5") => "fifteen",
            Some("6") => "sixteen",
            Some("7") => "seventeen",
            Some("8") => "eighteen",
            Some("9") => "nineteen",
            _ => panic!("invalid ones digit in handle_tens"),
        },
        Some("2") => "twenty",
        Some("3") => "thirty",
        Some("4") => "forty",
        Some("5") => "fifty",
        Some("6") => "sixty",
        Some("7") => "seventy",
        Some("8") => "eighty",
        Some("9") => "ninety",
        _ => panic!("invalid first char in num_str: {}", num_str),
    };

    if !num_str.starts_with('1') && !num_str.ends_with('0') {
        format!("{}-{}", tens_name, handle_ones(num_str.get(1..2).unwrap()))
    } else {
        tens_name.to_string()
    }
}

fn handle_hundreds(num_str: &str) -> String {
    let mut hundreds_str: String = handle_ones(num_str.get(0..1).unwrap());
    hundreds_str.push_str(" hundred");

    if num_str.get(1..2).unwrap() != "0" {
        hundreds_str.push_str(" ");
        hundreds_str.push_str(&handle_tens(num_str.get(1..3).unwrap()));
    }
    hundreds_str
}

fn handle_thousands(num_str: &str) -> String {
    let mut thousands_str = String::with_capacity(6);

    // handle leading nums before hundreds digit
    if num_str.len() == 6 {
        let leading_hundreds_str = handle_hundreds(&num_str.get(0..3).unwrap());
        thousands_str.push_str(&leading_hundreds_str);
    } else if num_str.len() == 5 {
        let leading_tens_str = handle_tens(&num_str.get(0..2).unwrap());
        thousands_str.push_str(&leading_tens_str);
    } else if num_str.len() == 4 {
        let leading_ones_str = handle_ones(&num_str.get(0..1).unwrap());
        thousands_str.push_str(&leading_ones_str);
    }
    thousands_str.push_str(" thousand");

    let hundreds_digit_index = num_str.len() - 3;
    if num_str.get(hundreds_digit_index..hundreds_digit_index + 1).unwrap() != "0" {
        thousands_str.push_str(" ");
        // TODO: set range below based on num_str.len()
        thousands_str.push_str(&handle_hundreds(num_str.get(num_str.len() - 3..num_str.len()).unwrap()));
    }
    thousands_str
}

fn handle_millions(num_str: &str) -> String {
    String::new()
}

fn handle_billions(num_str: &str) -> String {
    String::new()
}

fn handle_trillions(num_str: &str) -> String {
    String::new()
}

fn handle_quadrillions(num_str: &str) -> String {
    String::new()
}

fn handle_quintillions(num_str: &str) -> String {
    String::new()
}