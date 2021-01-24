pub fn reply(message: &str) -> &str {
    let mut response = "Whatever.";
    let contains_alphabet = message.chars().any(char::is_alphabetic);
    let is_uppercase = message.trim() != "" && message.to_uppercase() == message;
    let is_question = message.trim().ends_with("?");
    
    if is_question {
        if is_uppercase && contains_alphabet {
            response = "Calm down, I know what I'm doing!";
        } else {
            response = "Sure.";
        }
    } else if is_uppercase && contains_alphabet {
        response = "Whoa, chill out!";
    } else if message.trim() == "" {
        response = "Fine. Be that way!";
    }

    response
}
