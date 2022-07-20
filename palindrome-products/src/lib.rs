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

    // pub fn insert(&mut self, (_from, _to): (u64, u64)) {}
}

fn is_palindrome(num: u64) -> bool {
    let str: String = num.to_string();
    let rev_str: String = str.chars().rev().collect();
    str == rev_str
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }
    let mut result: Option<(Palindrome, Palindrome)> = None;
    let should_merge_sort = (max - min) / 2 >= min;

    let min_products_max_range = if should_merge_sort { (max - min) / 2 } else { max };
    let min_products = find_palindrome_products_in_range(min, min_products_max_range);
    if !min_products.is_empty() {
        let min_palindrome = Palindrome::new(*min_products.iter().min().unwrap()).unwrap();
        let max_palindrome = Palindrome::new(*min_products.iter().max().unwrap()).unwrap();
        result = Some((min_palindrome, max_palindrome));
    }

    if should_merge_sort {
        let max_products = find_palindrome_products_in_range(max / 2, max);
        if !max_products.is_empty() {
            let min_product = if min_products.is_empty() {
                *max_products.iter().min().unwrap()
            } else {
                *min_products.iter().min().unwrap()
            };
            let min_palindrome = Palindrome::new(min_product).unwrap();
            let max_palindrome = Palindrome::new(*max_products.iter().max().unwrap()).unwrap();
            result = Some((min_palindrome, max_palindrome));
        }
    }

    result
}

fn find_palindrome_products_in_range(min: u64, max: u64) -> Vec<u64> {
    (min..=max)
        .flat_map(|x| (x..=max).map(move |y| x * y))
        .filter(|&x| is_palindrome(x))
        .collect::<Vec<u64>>()
}
