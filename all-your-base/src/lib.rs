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
    if from_base == 0 {
        Err(Error::InvalidInputBase)
    } else if to_base == 0 {
        Err(Error::InvalidOutputBase)
    } else {
        // let mut num_from_digits = calc_num_from_digits(number, from_base);
        let mut result_vec: Vec<u32> = Vec::new();
        for i in 0..=number.len() - 1 {
            let num = number[i];
            if num >= from_base {
                return Err(Error::InvalidDigit(num));
            } else {
                let exp = (number.len() - 1 - i) as u32;
                result_vec.push(num * to_base.pow(exp))
            }
        }
        Ok(result_vec)
    }
}

// fn calc_num_from_digits(number: &[u32], from_base: u32) -> u32 {
//     let mut num = 0;

//     for digit in number.iter() {
//         num = from_base * num + digit;
//     }
//     num
// }

// fn calc_digits_from_num(mut number: u32, to_base: u32) -> Vec<u32> {
//     let mut digits: Vec<u32> = Vec::new();
//     while number > 0 {
//         digits.push()
//     }

//     digits
// }