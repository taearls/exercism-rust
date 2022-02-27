pub fn number(user_number: &str) -> Option<String> {
    let mut digits = user_number
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();

    if digits.len() == 11 {
        if digits.get(0..1).unwrap() != "1" {
            return None;
        } else {
            digits.drain(0..1);
        }
    }

    if digits.len() != 10 {
        return None;
    }

    let mut is_valid = true;
    let result = digits
        .chars()
        .enumerate()
        .map(|(index, digit)| {
            if [0, 3].contains(&index) && ['0', '1'].contains(&digit) {
                is_valid = false;
            }
            digit
        })
        .collect::<String>();
    if is_valid {
        Some(result)
    } else {
        None
    }
}
