use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        HashMap::from([
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

pub struct Roman { num: String }

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.num)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        unimplemented!("Construct a Roman object from the '{}' number", num);
    }
}
