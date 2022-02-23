pub fn is_valid(code: &str) -> bool {
    let filtered_digits = code
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();
    if filtered_digits.len() < 2
        || filtered_digits.len() != code.chars().filter(|c| *c != ' ').count()
    {
        return false;
    }

    filtered_digits
        .iter()
        .enumerate()
        .map(|(index, &digit)| {
            let mut digit = digit;
            if filtered_digits.len() % 2 == 0 {
                if index % 2 == 0 {
                    digit *= 2;
                    if digit > 9 {
                        digit -= 9;
                    }
                }
            } else if index % 2 == 1 {
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
            }
            digit
        })
        .sum::<u32>()
        % 10
        == 0
}
