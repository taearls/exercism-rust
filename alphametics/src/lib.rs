use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    if !input.contains(" == ") { return None; }

    let mut result: HashMap<char, u8> = HashMap::new();

    let split: Vec<&str> = input.split(" == ").collect();

    // safe since we validate that " == " is in this &str at top of fn
    let ls = unsafe { split.get_unchecked(0) };
    let ls_words: Vec<&str> = ls.split(" + ").collect();

    let rs = unsafe { split.get_unchecked(1) };

    
    // set default values to check 0 - 9
    // filter out 0 for leading chars in multi-digit nums 

    // iterate through all cases, starting from lower nums for leading digits

    // figure out which chars I need to check
    for c in input.chars() {
        if !result.contains_key(&c) {
            result.insert(c, 0);
        }
    }
    if !(1..=10).contains(&result.len()) {
        return None;
    }
    Some(result)
}
