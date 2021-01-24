pub fn brackets_are_balanced(string: &str) -> bool {
    let mut is_balanced = true;
    let mut left_chars_to_match = String::new();
    
    for c in string.chars() {
        let (index_brace, index_bracket, index_parentheses) = find_rightmost_left_char_indexes(&left_chars_to_match);
        
        // if left_char found, add it to the left_chars_to_match string we're checking against, 
        // and change is_balanced to false

        // if right_char found, remove the rightmost occurrence of the char in the string. 
        // if matching string length is 0, set is_balanced to true
        match c {
            '{' => {
                left_chars_to_match.push('{');
                is_balanced = false;
            },
            '}' => {
                if index_bracket == None || index_brace > index_bracket || index_parentheses > index_bracket {
                    is_balanced = false;
                    break;
                } else {
                    left_chars_to_match.remove(index_bracket.unwrap());
                    if left_chars_to_match.len() == 0 {
                        is_balanced = true;
                    }
                }
            },
            '[' => {
                left_chars_to_match.push('[');
                is_balanced = false;
            },
            ']' => {
                if index_brace == None || index_bracket > index_brace || index_parentheses > index_brace {
                    is_balanced = false;
                    break;
                } else {
                    left_chars_to_match.remove(index_brace.unwrap());
                    if left_chars_to_match.len() == 0 {
                        is_balanced = true;
                    }
                }
            },
            '(' => {
                left_chars_to_match.push('(');
                is_balanced = false;
            },
            ')' => {
                if index_parentheses == None || index_brace > index_parentheses || index_bracket > index_parentheses {
                    is_balanced = false;
                    break;
                } else {
                    left_chars_to_match.remove(index_parentheses.unwrap());
                    if left_chars_to_match.len() == 0 {
                        is_balanced = true;
                    }
                }
            },
            _ => ()
        }
    }
    is_balanced && left_chars_to_match.len() == 0
}

// returns the rightmost occurences of each right_char, or None if no match is found.
fn find_rightmost_left_char_indexes(left_chars_to_match: &str) -> (Option<usize>, Option<usize>, Option<usize>) {
    let index_brace = left_chars_to_match.rfind('[');
    let index_bracket = left_chars_to_match.rfind('{');
    let index_parentheses = left_chars_to_match.rfind('(');

    (index_brace, index_bracket, index_parentheses)
}