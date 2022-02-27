#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    } else if string_digits.is_empty() || span == 0 {
        return Ok(1);
    }

    let invalid_chars: Vec<char> = string_digits.chars().filter(|c| !c.is_numeric()).collect();
    if !invalid_chars.is_empty() {
        let invalid_char = invalid_chars.get(0).unwrap();
        return Err(Error::InvalidDigit(*invalid_char));
    }

    let mut product: u64 = 0;

    let digits = string_digits
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();

    for slice in digits.windows(span) {
        let slice_product: u64 = slice.iter().product();
        if slice_product > product {
            product = slice_product;
        }
    }

    Ok(product)
}
