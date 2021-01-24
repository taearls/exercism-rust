pub fn raindrops(n: u32) -> String {
    let mut str = String::new();

    if n % 3 == 0 {
        str.push_str("Pling")
    }
    if n % 5 == 0 {
        str.push_str("Plang")
    }
    if n % 7 == 0 {
        str.push_str("Plong")
    }

    // if there are no characters in str, assign the string to the value of the input n
    if str.chars().count() == 0 {
        str = String::from(n.to_string());
    }
    str
}
