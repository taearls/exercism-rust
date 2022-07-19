use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    inner: u64,
}

impl Palindrome {
    pub fn new(number: u64) -> Option<Palindrome> {
        if is_palindrome(number) {
            Some(Palindrome { inner: number })
        } else {
            None
        }
    }

    pub fn into_inner(&self) -> u64 {
        self.inner
    }

}

fn is_palindrome(num: u64) -> bool {
    let str: String = num.to_string();
    let rev_str: String = str.chars().rev().collect();
    str == rev_str
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let palindromes: Vec<u64> = (min..=max)
        .filter(|num| is_palindrome(*num))
        .collect();
        
    if min > max {
        return None;
    }
    let mut min_iter = min;
    let mut min_palindrome: Option<Palindrome> = None;
    let mut max_palindrome: Option<Palindrome> = None;
    while min_iter < max {
        for i in min_iter..=max {
            if let Some(new_palindrome) = Palindrome::new(min_iter * i) {
                if min_palindrome.is_none() {
                    min_palindrome = Some(new_palindrome);
                } else if max_palindrome.is_none() {
                    match min_palindrome
                        .as_mut()
                        .unwrap()
                        .into_inner()
                        .cmp(&new_palindrome.into_inner())
                    {
                        Ordering::Less | Ordering::Equal => {
                            max_palindrome = Some(new_palindrome);
                        }
                        Ordering::Greater => {
                            max_palindrome = min_palindrome;
                            min_palindrome = Some(new_palindrome);
                        }
                    }
                } else if new_palindrome.into_inner()
                    < min_palindrome.as_mut().unwrap().into_inner()
                {
                    min_palindrome = Some(new_palindrome);
                } else if new_palindrome.into_inner()
                    > max_palindrome.as_mut().unwrap().into_inner()
                {
                    max_palindrome = Some(new_palindrome);
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
