use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            value: a * b,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, _a: u64, _b: u64) {}

    pub fn is_palindrome(num: u64) -> bool {
        let str: String = num.to_string();
        let rev_str: String = str.chars().rev().collect();
        str == rev_str
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max { 
        return None; 
    }
    let mut min_iter = min;
    let mut min_palindrome: Option<Palindrome> = None;
    let mut max_palindrome: Option<Palindrome> = None;
    while min_iter < max {
        for i in min_iter..=max {
            if Palindrome::is_palindrome(min_iter * i) {
                let new_palindrome = Palindrome::new(min_iter, i);
                if min_palindrome.is_none() {
                    min_palindrome = Some(new_palindrome);
                } else if max_palindrome.is_none() {
                    match min_palindrome.as_mut().unwrap().value().cmp(&new_palindrome.value()) {
                        Ordering::Less | Ordering::Equal => {
                            max_palindrome = Some(new_palindrome);
                        },
                        Ordering::Greater => {
                            max_palindrome = min_palindrome;
                            min_palindrome = Some(new_palindrome);
                        },
                    }
                } else {
                    if new_palindrome.value() < min_palindrome.as_mut().unwrap().value() {
                        min_palindrome = Some(new_palindrome);
                    } else if new_palindrome.value() > max_palindrome.as_mut().unwrap().value() {
                        max_palindrome = Some(new_palindrome);
                    }
                }
            }
        }
        min_iter += 1;
    }
    
    if min_palindrome.is_none() || max_palindrome.is_none() {
        return None;
    }
    Some((min_palindrome.unwrap(), max_palindrome.unwrap()))
}
