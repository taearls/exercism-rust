use num_bigint::{BigInt, Sign};

pub struct Decimal {
    decimal_factor: usize,
    raw_value: BigInt,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        if input.len() < 2 || input.matches('.').count() != 1 {
            return None;
        }

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

        let decimal_factor = input.len() - 1 - input.find('.').unwrap();

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
