pub fn encode(n: u64) -> String {
    let mut result = String::new();

    let mut digit_vec: Vec<String> = Vec::new();
    let mut temp_str = String::with_capacity(3);

    for digit in n.to_string().chars() {
        temp_str.push(digit);
        if temp_str.len() == temp_str.capacity() {
            digit_vec.push(temp_str.clone());
            temp_str.clear();
        }
    }
    if !temp_str.is_empty() {
        digit_vec.push(temp_str.clone());
        temp_str.clear();
    }
    for digit in digit_vec {
        result.push_str(&say_num_str(&digit))
    }
    result
}

fn say_num_str(digit: &str) -> String {
    match digit {
        "0" => "zero".to_string(),
        "1" => "one".to_string(),
        "2" => "two".to_string(),
        "3" => "three".to_string(),
        "4" => "four".to_string(),
        "5" => "five".to_string(),
        "6" => "six".to_string(),
        "7" => "seven".to_string(),
        "8" => "eight".to_string(),
        "9" => "nine".to_string(),
        "10" => "ten".to_string(),
        "11" => "eleven".to_string(),
        "12" => "twelve".to_string(),
        "13" => "thirteen".to_string(),
        "14" => "fourteen".to_string(),
        "15" => "fifteen".to_string(),
        "16" => "sixteen".to_string(),
        "17" => "seventeen".to_string(),
        "18" => "eighteen".to_string(),
        "19" => "nineteen".to_string(),
        _ if digit.len() == 2 && digit.starts_with('2') => handle_tens(digit, "twenty"),
        _ if digit.len() == 2 && digit.starts_with('3') => handle_tens(digit, "thirty"),
        _ if digit.len() == 2 && digit.starts_with('4') => handle_tens(digit, "forty"),
        _ if digit.len() == 2 && digit.starts_with('5') => handle_tens(digit, "fifty"),
        _ if digit.len() == 2 && digit.starts_with('6') => handle_tens(digit, "sixty"),
        _ if digit.len() == 2 && digit.starts_with('7') => handle_tens(digit, "seventy"),
        _ if digit.len() == 2 && digit.starts_with('8') => handle_tens(digit, "eighty"),
        _ if digit.len() == 2 && digit.starts_with('9') => handle_tens(digit, "ninety"),
        _ => panic!("invalid digit: {}", digit),
    }
}

fn handle_tens(num_str: &str, tens_name: &str) -> String {
    if !num_str.ends_with('0') {
        format!(
            "{}-{}",
            tens_name,
            say_num_str(num_str.chars().nth(1).unwrap().to_string().as_str())
        )
    } else {
        tens_name.to_string()
    }
}
