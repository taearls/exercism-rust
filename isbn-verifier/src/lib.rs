/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {  
    let isbn_no_hyphens: String = isbn.replace('-', "");
    let mut isbn_sum: u16 = 0;
    
    if isbn_no_hyphens.len() == 10 {
        
        for (index, c) in isbn_no_hyphens.char_indices() {
            if index == isbn_no_hyphens.len() - 1 && c.to_ascii_lowercase() == 'x' {
                isbn_sum += 10;
            } else if !c.is_numeric() {
                return false;
            } else {
                let digit = c.to_digit(10).unwrap() as u16;
                let multiplier = (10 - index) as u16;
                isbn_sum += digit * multiplier;
            }
        }

    } 
    isbn_sum != 0 && isbn_sum % 11 == 0
}
