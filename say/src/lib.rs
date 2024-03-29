pub fn encode(n: u64) -> String {
    let num_str = n.to_string();
    match num_str.len() {
        1 => handle_ones(&num_str),
        2 => handle_tens(&num_str),
        3 => handle_hundreds(&num_str),
        capacity @ 4..=6 => handle_scale(&num_str, capacity, "thousand"),
        capacity @ 7..=9 => handle_scale(&num_str, capacity, "million"),
        capacity @ 10..=12 => handle_scale(&num_str, capacity, "billion"),
        capacity @ 13..=15 => handle_scale(&num_str, capacity, "trillion"),
        capacity @ 16..=18 => handle_scale(&num_str, capacity, "quadrillion"),
        capacity @ 19..=20 => handle_scale(&num_str, capacity, "quintillion"),
        _ => panic!("invalid num_str: {}", &num_str),
    }
}

fn handle_ones(num_str: &str) -> String {
    if num_str.len() != 1 {
        panic!("invalid str length in handle_ones: {}", num_str)
    }
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
    if num_str.len() != 2 {
        panic!("invalid str length in handle_tens: {}", num_str)
    }
    if num_str.starts_with('0') {
        if &num_str[1..2] == "0" {
            return String::new();
        } else {
            return handle_ones(&num_str[1..2]);
        }
    }
    let tens_name = match &num_str[0..1] {
        "1" => match &num_str[1..2] {
            "0" => "ten",
            "1" => "eleven",
            "2" => "twelve",
            "3" => "thirteen",
            "4" => "fourteen",
            "5" => "fifteen",
            "6" => "sixteen",
            "7" => "seventeen",
            "8" => "eighteen",
            "9" => "nineteen",
            _ => panic!("invalid ones digit in handle_tens"),
        },
        "2" => "twenty",
        "3" => "thirty",
        "4" => "forty",
        "5" => "fifty",
        "6" => "sixty",
        "7" => "seventy",
        "8" => "eighty",
        "9" => "ninety",
        _ => panic!("invalid first char in num_str: {}", num_str),
    };

    if !num_str.starts_with('1') && !num_str.ends_with('0') {
        format!("{}-{}", tens_name, handle_ones(&num_str[1..2]))
    } else {
        tens_name.to_string()
    }
}

fn handle_hundreds(num_str: &str) -> String {
    if num_str.len() != 3 {
        panic!("invalid str length in handle_hundreds: {}", num_str)
    }

    // handle leading zeroes
    if num_str.starts_with('0') {
        if &num_str[1..2] == "0" {
            if &num_str[2..3] == "0" {
                return String::new();
            } else {
                return handle_ones(&num_str[2..3]);
            }
        } else {
            return handle_tens(&num_str[1..3]);
        }
    }
    let mut hundreds_str = String::with_capacity(3);

    let ones_str: String = handle_ones(&num_str[0..1]);
    hundreds_str.push_str(&ones_str);
    hundreds_str.push_str(" hundred");

    let tens_str = &handle_tens(&num_str[1..3]);
    if !tens_str.is_empty() {
        hundreds_str.push(' ');
    }

    hundreds_str.push_str(tens_str);
    hundreds_str
}

fn handle_scale(num_str: &str, capacity: usize, scale_name: &str) -> String {
    let mut scale_str = String::with_capacity(capacity);
    let remainder = capacity.checked_rem(3).unwrap();

    // 9, rem 0 -> 3 leading digits
    // 8, rem 1 -> 2 leading digits
    // 7, rem 2 -> 1 leading digit
    let leading_digit_count = match remainder {
        0 => 3,
        _ => remainder,
    };
    match leading_digit_count {
        1 => {
            // ex: 1_000
            let leading_ones_str = handle_ones(&num_str.get(0..1).unwrap());
            scale_str.push_str(&leading_ones_str);
        }
        2 => {
            // ex: 10_000
            let leading_tens_str = handle_tens(&num_str.get(0..2).unwrap());
            scale_str.push_str(&leading_tens_str);
        }
        3 => {
            // ex: 100_000
            let leading_hundreds_str = handle_hundreds(&num_str.get(0..3).unwrap());
            scale_str.push_str(&leading_hundreds_str);
        }
        _ => unreachable!("leading_digit_count can only be 1, 2, or 3"),
    }

    // ex:
    // 9, rem 0 -> 9 - 3 = 6
    // 8, rem 2 -> 8 - 2 = 6
    // 7, rem 1 -> 7 - 1 = 6
    let mut new_capacity = capacity - leading_digit_count;

    let new_starting_index = capacity - new_capacity;
    let mut new_num_str = num_str.get(new_starting_index..num_str.len()).unwrap();

    // trim out leading digits which are a series of zeroes
    loop {
        let leading_digits = new_num_str.get(0..3).unwrap();
        if leading_digits != "000" || new_num_str.len() == 3 {
            break;
        }
        new_num_str = &new_num_str[3..];
        new_capacity -= 3;
    }
    scale_str.push(' ');
    scale_str.push_str(scale_name);
    scale_str.push(' ');

    if new_capacity > 3 {
        let new_scale_name = match new_num_str.len() {
            19..=20 => "quintillion",
            16..=18 => "quadrillion",
            13..=15 => "trillion",
            10..=12 => "billion",
            7..=9 => "million",
            4..=6 => "thousand",
            _ => panic!("invalid length in handle_scale: {}", new_num_str),
        };
        scale_str.push_str(&handle_scale(new_num_str, new_capacity, new_scale_name));
    } else {
        let hundreds_str = &handle_hundreds(new_num_str);

        // remove ending space if nothing after scale_str calls
        if hundreds_str.is_empty() {
            scale_str.pop();
        } else {
            scale_str.push_str(hundreds_str);
        }
    }

    scale_str
}
