use std::{
    cmp::{Ord, Ordering},
    ops::{Add, Mul, Sub},
};

use num_bigint::{BigInt, Sign};

#[derive(Eq, Debug)]
pub struct Decimal {
    decimal_factor: usize,
    raw_value: BigInt,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let sign = if input.starts_with('-') {
            Sign::Minus
        } else {
            Sign::Plus
        };
        let digits: Vec<u32> = input
            .split('.')
            .flat_map(|digit| {
                digit
                    .as_bytes()
                    .chunks(32)
                    .filter_map(|chunks| match std::str::from_utf8(chunks) {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    })
                    .filter_map(|digit| match digit.parse::<u32>() {
                        Ok(digit) => Some(digit),
                        Err(_) => None,
                    })
                    .collect::<Vec<u32>>()
            })
            .collect();
        let raw_value = BigInt::new(sign, digits);

        let decimal_factor = match input.find('.') {
            Some(index) => input.len() - 1 - index,
            None => 0,
        };

        Some(Decimal {
            raw_value,
            decimal_factor,
        })
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.raw_value == other.raw_value && self.decimal_factor == other.decimal_factor
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.raw_value.cmp(&other.raw_value) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.decimal_factor.cmp(&other.decimal_factor),
        }
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, rhs: Self) -> Self::Output {
        let decimal_factor = get_decimal_factor(&self, &rhs);

        Decimal {
            raw_value: self.raw_value + rhs.raw_value,
            decimal_factor,
        }
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Self) -> Self::Output {
        let decimal_factor = get_decimal_factor(&self, &rhs);

        Decimal {
            raw_value: self.raw_value - rhs.raw_value,
            decimal_factor,
        }
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Self) -> Self::Output {
        let decimal_factor = get_decimal_factor(&self, &rhs);
        
        Decimal {
            raw_value: self.raw_value * rhs.raw_value,
            decimal_factor,
        }
    }
}

fn get_decimal_factor(lhs: &Decimal, rhs: &Decimal) -> usize {
    if rhs.decimal_factor == 0 || lhs.decimal_factor <= rhs.decimal_factor {
        lhs.decimal_factor
    } else {
        rhs.decimal_factor
    }
}
