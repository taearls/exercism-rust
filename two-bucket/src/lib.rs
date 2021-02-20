use std::cmp::{min, Ordering};
use std::collections::HashMap;

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
    // define default values
    bucket_amounts_hm.insert(1, 0);
    bucket_amounts_hm.insert(2, 0);

    let mut moves: u8 = 0;
    loop {
        moves += 1;
        pour(
            &mut bucket_amounts_hm,
            capacity_1,
            capacity_2,
            goal,
            start_bucket,
        );
        if check_solved(&bucket_amounts_hm, goal) {
            break;
        }
    }
    get_result_bucket_stats(moves, &bucket_amounts_hm, goal)
}

fn pour(
    bucket_amounts_hm: &mut HashMap<usize, u8>,
    bucket_one_capacity: u8,
    bucket_two_capacity: u8,
    goal: u8,
    start_bucket: &Bucket,
) {
    // three scenarios for each bucket amount
    // empty, full, or in between
    // one bucket has to be either 0 or full, or else this is not solvable

    let new_bucket_amounts = match get_bucket_amounts(bucket_amounts_hm) {
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
                    // if filling bucket one reaches the goal, let's do that immediately
                    if bucket_one_capacity == goal {
                        (bucket_one_capacity, bucket_two_amount)
                    } else {
                        // handles if bucket one capacity is bigger than bucket two capacity
                        let poured_amount = min(bucket_one_capacity, bucket_two_amount);
                        (poured_amount, bucket_two_amount - poured_amount)
                    }
                }
            }
        }
        (bucket_one_amount, 0) if bucket_one_amount == bucket_one_capacity => {
            match *start_bucket {
                // if we started with bucket one being filled, can't have bucket two empty and bucket one full
                Bucket::One => {
                    // if filling bucket two reaches the goal, let's do that immediately
                    if bucket_two_capacity == goal {
                        (bucket_one_amount, bucket_two_capacity)
                    } else {
                        // handles if bucket two capacity is bigger than bucket one capacity
                        let poured_amount = min(bucket_one_amount, bucket_two_capacity);
                        (bucket_one_amount - poured_amount, poured_amount)
                    }
                }
                Bucket::Two => (0, 0),
            }
        }
        (bucket_one_amount, bucket_two_amount) if bucket_one_amount == bucket_one_capacity => {
            match bucket_one_capacity.cmp(&bucket_two_capacity) {
                Ordering::Equal => (0, 0),
                Ordering::Greater => {
                    let poured_amount = bucket_one_capacity - bucket_two_amount;
                    let bucket_one_remainder = bucket_one_amount - poured_amount;

                    if *start_bucket == Bucket::Two {
                        (0, bucket_two_amount)
                    } else if bucket_one_remainder == bucket_two_capacity {
                        (bucket_one_amount, 0)
                    } else {
                        (
                            bucket_one_amount + poured_amount,
                            bucket_two_amount - poured_amount,
                        )
                    }
                }
                Ordering::Less => match *start_bucket {
                    Bucket::One => {
                        // handles if remaining space in bucket two is bigger than bucket one
                        let poured_amount =
                            min(bucket_two_capacity - bucket_two_amount, bucket_one_amount);
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
                        // handles if remaining space in bucket two is bigger than bucket one
                        let poured_amount =
                            min(bucket_two_capacity - bucket_two_amount, bucket_one_amount);
                        (
                            bucket_one_amount - poured_amount,
                            bucket_two_amount + poured_amount,
                        )
                    }
                },
                Ordering::Less => {
                    let poured_amount = bucket_one_capacity - bucket_one_amount;
                    let bucket_two_remainder = bucket_two_amount - poured_amount;

                    if *start_bucket == Bucket::One {
                        (bucket_one_amount, 0)
                    } else if bucket_two_remainder == bucket_one_capacity {
                        (0, bucket_two_amount)
                    } else {
                        (
                            bucket_one_amount + poured_amount,
                            bucket_two_amount - poured_amount,
                        )
                    }
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
        _ => (0, 0),
    };
    bucket_amounts_hm.insert(1, new_bucket_amounts.0);
    bucket_amounts_hm.insert(2, new_bucket_amounts.1);
}

// checks if bucket one amount or bucket two amount matches the goal
// if bucket amounts are (0, 0) we've determined we can't solve it
fn check_solved(bucket_amounts_hm: &HashMap<usize, u8>, goal: u8) -> bool {
    let (bucket_one_amount, bucket_two_amount) = get_bucket_amounts(bucket_amounts_hm);

    bucket_one_amount == goal
        || bucket_two_amount == goal
        || (bucket_one_amount == 0 && bucket_two_amount == 0)
}

fn get_result_bucket_stats(
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
