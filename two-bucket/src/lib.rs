use std::cmp::{min, Ordering};
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut bucket_amounts_hm: HashMap<usize, u8> = HashMap::new();
    bucket_amounts_hm.insert(1, 0);
    bucket_amounts_hm.insert(2, 0);

    let mut moves: u8 = 0;
    println!("start_bucket: {}", start_bucket);
    loop {
        moves += 1;
        println!("move #{}", moves);
        pour(&mut bucket_amounts_hm, capacity_1, capacity_2, start_bucket);
        if check_solved(&bucket_amounts_hm, goal) {
            break;
        }
    }
    get_result_stats(moves, &bucket_amounts_hm, goal)
}

// TEST 1 (3, 5, 1, &Bucket::One) -> 4
//   (3, 0)
//   (0, 3)
//   (3, 3)
//   (1, 5)

// TEST 2 (3, 5, 1, &Bucket::Two) -> 8
//   (0, 5)
//   (3, 2) -> bucket 1 + bucket 2 = bucket 2 capacity
//   (0, 2)
//   (2, 0)
//   (2, 5)
//   (3, 4)
//   (0, 4)
//   (3, 1)
fn pour(
    bucket_amounts_hm: &mut HashMap<usize, u8>,
    bucket_one_capacity: u8,
    bucket_two_capacity: u8,
    start_bucket: &Bucket,
) {
    // three scenarios for each bucket amount
    // empty, full, or in between
    // consider all 9 cases

    let new_bucket_amounts = match get_bucket_amounts(bucket_amounts_hm) {
        // THIS SHOULDNT HAPPEN
        (bucket_one_amount, bucket_two_amount)
            if bucket_one_amount == bucket_one_capacity
                && bucket_two_amount == bucket_two_capacity =>
        {
            (0, 0)
        }

        (0, 0) => {
            // handles first turn
            match *start_bucket {
                Bucket::One => (bucket_one_capacity, 0),
                Bucket::Two => (0, bucket_two_capacity),
            }
        }
        (0, bucket_two_amount) if bucket_two_amount == bucket_two_capacity => {
            match *start_bucket {
                // if we started with bucket one being filled, can't have bucket one empty and bucket one full
                Bucket::One => (0, 0),
                Bucket::Two => {
                    let poured_amount = min(bucket_one_capacity, bucket_two_amount);
                    (poured_amount, bucket_two_amount - poured_amount)
                }
            }
        }
        (bucket_one_amount, 0) if bucket_one_amount == bucket_one_capacity => {
            match *start_bucket {
                // if we started with bucket one being filled, can't have bucket two empty and bucket one full
                Bucket::One => {
                    let poured_amount = min(bucket_one_amount, bucket_two_capacity);
                    (bucket_one_amount - poured_amount, poured_amount)
                }
                Bucket::Two => (0, 0),
            }
        }
        (bucket_one_amount, bucket_two_amount) if bucket_one_amount == bucket_one_capacity => {
            match bucket_one_capacity.cmp(&bucket_two_capacity) {
                Ordering::Equal => (0, 0),
                Ordering::Greater => {
                    let poured_amount = bucket_one_capacity - bucket_two_amount;
                    (
                        bucket_one_amount + poured_amount,
                        bucket_two_amount - poured_amount,
                    )
                }
                Ordering::Less => match *start_bucket {
                    Bucket::One => {
                        let poured_amount = bucket_two_capacity - bucket_one_amount;
                        (
                            bucket_one_amount - poured_amount,
                            bucket_two_amount + poured_amount,
                        )
                    }
                    Bucket::Two => (0, bucket_two_amount),
                },
            }
        }
        (bucket_one_amount, bucket_two_amount) if bucket_two_amount == bucket_two_capacity => {
            match bucket_one_capacity.cmp(&bucket_two_capacity) {
                Ordering::Equal => (0, 0),
                Ordering::Greater => match *start_bucket {
                    Bucket::One => (bucket_one_amount, 0),
                    Bucket::Two => {
                        let poured_amount = bucket_two_capacity - bucket_two_amount;
                        (
                            bucket_one_amount - poured_amount,
                            bucket_two_amount + poured_amount,
                        )
                    }
                },
                Ordering::Less => {
                    let poured_amount = bucket_one_capacity - bucket_one_amount;
                    (
                        bucket_one_amount + poured_amount,
                        bucket_two_amount - poured_amount,
                    )
                }
            }
        }

        (0, bucket_two_amount) => match *start_bucket {
            Bucket::One => (bucket_one_capacity, bucket_two_amount),
            Bucket::Two => {
                // handles if bucket one capacity is smaller than amount in bucket two
                let pour_amount = min(bucket_one_capacity, bucket_two_amount);
                (pour_amount, bucket_two_amount - pour_amount)
            }
        },
        (bucket_one_amount, 0) => match *start_bucket {
            Bucket::One => {
                // handles if bucket two capacity is smaller than amount in bucket one
                let pour_amount = min(bucket_one_amount, bucket_two_capacity);
                (bucket_one_amount - pour_amount, pour_amount)
            }
            Bucket::Two => (bucket_one_amount, bucket_two_capacity),
        },

        // catch all. not sure if I need to add logic for this
        _ => (0, 0),
    };

    bucket_amounts_hm.insert(1, new_bucket_amounts.0);
    bucket_amounts_hm.insert(2, new_bucket_amounts.1);
    println!("({}, {})", bucket_amounts_hm[&1], bucket_amounts_hm[&2]);
}

fn check_solved(bucket_amounts_hm: &HashMap<usize, u8>, goal: u8) -> bool {
    let (bucket_one_amount, bucket_two_amount) = get_bucket_amounts(bucket_amounts_hm);

    bucket_one_amount == goal
        || bucket_two_amount == goal
        || (bucket_one_amount == 0 && bucket_two_amount == 0)
}

fn get_result_stats(
    moves: u8,
    bucket_amounts_hm: &HashMap<usize, u8>,
    goal: u8,
) -> Option<BucketStats> {
    match get_bucket_amounts(&bucket_amounts_hm) {
        (bucket_one_amount, bucket_two_amount) if bucket_one_amount == goal => Some(BucketStats {
            moves,
            goal_bucket: Bucket::One,
            other_bucket: bucket_two_amount,
        }),
        (bucket_one_amount, bucket_two_amount) if bucket_two_amount == goal => Some(BucketStats {
            moves,
            goal_bucket: Bucket::Two,
            other_bucket: bucket_one_amount,
        }),
        _ => None,
    }
}

fn get_bucket_amounts(bucket_amounts_hm: &HashMap<usize, u8>) -> (u8, u8) {
    (bucket_amounts_hm[&1], bucket_amounts_hm[&2])
}

impl fmt::Display for Bucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bucket_name = match *self {
            Bucket::One => "Bucket One",
            Bucket::Two => "Bucket Two",
        };
        write!(f, "{}", bucket_name)
    }
}
