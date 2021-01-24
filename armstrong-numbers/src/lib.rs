pub fn is_armstrong_number(num: u32) -> bool {
    let mut armstrong_num: u32 = 0;
    let digit_count = num.to_string().len() as u32;
    for i in num.to_string().chars() {
        armstrong_num += (i.to_digit(10).unwrap()).pow(digit_count);
    }
    armstrong_num == num
}