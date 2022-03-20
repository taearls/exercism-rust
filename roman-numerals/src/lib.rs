use lazy_static::lazy_static;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Result};

lazy_static! {
    static ref HASHMAP: BTreeMap<u32, &'static str> = {
        BTreeMap::from([
            (1, "I"),
            (5, "V"),
            (10, "X"),
            (50, "L"),
            (100, "C"),
            (500, "D"),
            (1000, "M"),
        ])
    };
}

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut result = String::new();
        let mut num = num;
        for (key, value) in HASHMAP.iter().rev() {
            while &num >= key {
                let len = int_log10(num);

                let (lower_bound, upper_bound) = match len {
                    1 if num < 5 => (4, 5),
                    1 => (9, 10),
                    2 if num < 50 => (40, 50),
                    2 => (90, 100),
                    3 if key < &500 => (400, 500),
                    3 => (900, 1000),
                    _ => (*key, *key),
                };
                if (lower_bound..upper_bound).contains(&num) {
                    let mut new_str = String::new();
                    match lower_bound {
                        4 => new_str.push_str("IV"),
                        9 => new_str.push_str("IX"),
                        40 => {
                            let result = parse_lower_bound_roman_num(num, lower_bound, "XL");
                            new_str.push_str(&result);
                        }
                        90 => {
                            let result = parse_lower_bound_roman_num(num, lower_bound, "XC");
                            new_str.push_str(&result);
                        }
                        400 => {
                            let result = parse_lower_bound_roman_num(num, lower_bound, "CD");
                            new_str.push_str(&result);
                        }
                        900 => {
                            let result = parse_lower_bound_roman_num(num, lower_bound, "CM");
                            new_str.push_str(&result);
                        }
                        _ => {}
                    };
                    result += &new_str;
                    num = 0;
                } else {
                    result.push_str(value);
                    num -= key;
                }
            }
        }
        Self(result)
    }
}

fn parse_lower_bound_roman_num(num: u32, lower_bound: u32, initial: &str) -> String {
    let num = num - lower_bound;
    let mut end = String::new();
    if num > 0 {
        end = Roman::from(num).0;
    }
    format!("{initial}{end}")
}

fn int_log10<T>(mut i: T) -> u32
where
    T: std::ops::DivAssign + std::cmp::PartialOrd + From<u8> + Copy,
{
    let mut len = 0;
    let zero = T::from(0);
    let ten = T::from(10);

    while i > zero {
        i /= ten;
        len += 1;
    }

    len
}
