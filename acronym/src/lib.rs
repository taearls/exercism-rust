pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();    

    // if previous char is a space or hyphen, add it to result string.
    // if the char is uppercased, regardless of punctuation, add it to result string.
    // initialize previous_char as space so first letter is abbreviated automatically.
    let mut previous_char: char = ' ';
    for i in phrase.chars() {
            
        match previous_char {
            ' ' | '-' | '_' =>  {
                if i.is_ascii_alphabetic() {
                    result.push(i.to_ascii_uppercase());
                }
            },
            _ => {
                // two capitals in the same word should not both be counted.
                // only count capital chars after non capital chars
                if i.is_ascii_uppercase() && previous_char.is_ascii_lowercase() {
                    result.push(i);
                }
            },
        }
        previous_char = i;
    }
    result
}
