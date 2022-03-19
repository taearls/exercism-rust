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
            println!("key is: {key}, value is {value}");
            while &num >= key {
                result.push_str(value);
                num -= key;
            }
        }
        Self(result)
    }
}
