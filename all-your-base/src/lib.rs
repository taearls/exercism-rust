#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        Err(Error::InvalidInputBase)
    } else if to_base <= 1 {
        Err(Error::InvalidOutputBase)
    } else {
        let base_ten_value: u32 = calc_base_ten_value(number, from_base)?;
        Ok(calc_digits_from_base_ten(base_ten_value, to_base))
    }
}

fn calc_base_ten_value(number: &[u32], from_base: u32) -> Result<u32, Error> {
    let mut num = 0;

    for (index, digit) in number.iter().enumerate() {
        let exp = (number.len() - 1 - index) as u32;
        if digit >= &from_base {
            return Err(Error::InvalidDigit(*digit));
        }
        num += digit * (from_base.pow(exp));
    }
    Ok(num)
}

fn calc_digits_from_base_ten(mut base_ten_value: u32, to_base: u32) -> Vec<u32> {
    if base_ten_value == 0 {
        return vec![0];
    }

    let mut digits: Vec<u32> = Vec::new();

    while base_ten_value > 0 {
        let digit = base_ten_value % to_base;
        digits.push(digit);
        base_ten_value = (base_ten_value - digit) / to_base;
    }
    // we're pushing ones digit in first, but we need it the opposite way
    digits.reverse();
    digits
}
