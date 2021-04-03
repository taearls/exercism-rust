use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None, 
        _ => {
            let aliquot_sum = calc_aliquot_sum(num);

            let classification = match aliquot_sum.cmp(&num) {
                Ordering::Less => Classification::Deficient,
                Ordering::Equal => Classification::Perfect,
                Ordering::Greater => Classification::Abundant,
            };
            Some(classification)
        },
    }
}

fn calc_aliquot_sum(num: u64) -> u64 {
    let mut aliquot_sum = 0;
    
    for i in 1..=(num / 2) {
        if num % i == 0 {
            aliquot_sum += i;
        }
    }
    aliquot_sum
}
