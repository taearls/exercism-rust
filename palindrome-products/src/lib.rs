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

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }
    let mut min_product: Option<u64> = None;
    let mut max_product: Option<u64> = None;

    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            if (min_product.is_none() || product < min_product.unwrap()) && is_palindrome(product) {
                min_product = Some(product);
            } else if (max_product.is_none() || product > max_product.unwrap())
                && is_palindrome(product)
            {
                max_product = Some(product);
            }
        }
    }

    match (min_product, max_product) {
        (Some(min_product), Some(max_product)) => Some((
            Palindrome::new(min_product).unwrap(),
            Palindrome::new(max_product).unwrap(),
        )),
        _ => None,
    }
}

fn is_palindrome(num: u64) -> bool {
    let str: String = num.to_string();
    let rev_str: String = str.chars().rev().collect();
    str == rev_str
}
