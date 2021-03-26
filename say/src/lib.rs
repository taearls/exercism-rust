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
    println!("handle tens num_str: {}", num_str);
    if num_str.starts_with('0') {
        if &num_str[1..2] == "0" {
            return String::new();
        } else {
            println!("boom");
            return handle_ones(&num_str[1..2]);
        }
    }
    let tens_name = match num_str.get(0..1) {
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
    println!("num_str inside handle_hundreds: {}", num_str);

    // handle leading zeroes
    if num_str.starts_with('0') {
        if &num_str[1..2] == "0" {
            if &num_str[2..3] == "0" {
                return String::new();
            } else {
                return handle_ones(num_str);
            }
        } else {
            return handle_tens(num_str);
        }
    }

    let mut hundreds_str: String = handle_ones(num_str.get(0..1).unwrap());
    hundreds_str.push_str(" hundred");

    if num_str.get(1..2).unwrap() != "0" {
        hundreds_str.push_str(" ");
        hundreds_str.push_str(&handle_tens(num_str.get(1..3).unwrap()));
    }
    hundreds_str
}

fn handle_scale(num_str: &str, capacity: usize, scale_name: &str) -> String {
    // if num_str.starts_with('0') { return String::new() }
    let mut scale_str = String::with_capacity(capacity);

    println!("num_str.len() inside handle_scale: {}", num_str.len());
    println!(
        "scale_str.capacity() inside handle_scale: {}",
        scale_str.capacity()
    );
    let remainder = capacity.checked_rem(3).unwrap();

    // 9, rem 0 -> 3
    // 8, rem 1 -> 2
    // 7, rem 2 -> 1
    // 6
    match remainder {
        0 => {
            // ex: 100_000
            let leading_hundreds_str = handle_hundreds(&num_str.get(0..3).unwrap());
            scale_str.push_str(&leading_hundreds_str);
        }
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
        _ => unreachable!("remainder of division by 3 can only be 0, 1, or 2"),
    }

    // } else if num_str.len() == scale_str.capacity() - 1 {

    // } else if num_str.len() == scale_str.capacity() - 2 {

    // }
    scale_str.push_str(" ");
    scale_str.push_str(scale_name);

    // 9, rem 0 -> 3
    // 8, rem 1 -> 2
    // 7, rem 2 -> 1
    // 6

    // subtract 1 from capacity to make it 0 based
    let downgraded_scale_digit_index = capacity - 1 - (3 - remainder);

    println!(
        "downgraded_scale_digit_index: {}",
        downgraded_scale_digit_index
    );
    if num_str
        .get(downgraded_scale_digit_index..downgraded_scale_digit_index + 1)
        .unwrap()
        != "0"
    {
        scale_str.push_str(" ");

        let new_capacity = downgraded_scale_digit_index + 1;
        if new_capacity > 3 {
            let new_num_str = num_str
                .get(num_str.len() - new_capacity..num_str.len())
                .unwrap();
            let new_scale_name = match scale_name {
                "quintillion" => "quadrillion",
                "quadrillion" => "trillion",
                "trillion" => "billion",
                "billion" => "million",
                "million" => "thousand",
                _ => panic!("invalid scale name in handle_scale: {}", scale_name),
            };
            scale_str.push_str(&handle_scale(new_num_str, new_capacity, new_scale_name));
        } else {
            scale_str.push_str(&handle_hundreds(
                num_str
                    .get(downgraded_scale_digit_index..num_str.len())
                    .unwrap(),
            ));
        }
    }
    scale_str
}
