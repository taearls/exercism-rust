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

    if let Some(min_product) = min_product && let Some(max_product) = max_product {
        Some((
            Palindrome::new(min_product).unwrap(),
            Palindrome::new(max_product).unwrap(),
        ))
    } else {
        None
    }
}

// fn find_palindrome_products_in_range(min: u64, max: u64) -> Vec<u64> {
//     (min..=max)
//         .flat_map(|x| (x..=max).map(move |y| x * y))
//         .filter(|&x| is_palindrome(x))
//         .collect::<Vec<u64>>()
// }

// fn find_min_max_palindrome_products(min: u64, max: u64) -> Option<(u64, u64)> {
//     (min..=max)
//         .flat_map(|x| (x..=max).filter_map(move |y| if x * y > ))
//         .filter(|&x| is_palindrome(x))
//         .collect::<Vec<u64>>()
// }
