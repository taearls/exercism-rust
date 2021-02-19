use std::collections::HashMap;
use std::fmt;
use std::cmp::{min, Ordering};

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



// TEST 2 (3, 5, 1, &Bucket::Two) -> 8
// 2 (0, 5)
// 1 (3, 2) -> bucket 1 + bucket 2 = bucket 2 capacity
// 2 (0, 2)
//   (2, 0)
//   (2, 5)
//   (3, 4)
//   (0, 4)
//   (3, 1)

// TEST 1 (3, 5, 1, &Bucket::One) -> 4
// 1 (3, 0)
// 2 (0, 3)
// 1 (3, 3)
// 2 (1, 5)

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut bucket_amounts_hm: HashMap<usize, u8> = HashMap::new();

    // do initial move
    match *start_bucket {
        Bucket::One => {
            bucket_amounts_hm.insert(1, capacity_1);
            bucket_amounts_hm.insert(2, 0);
        }, 
        Bucket::Two => {
            bucket_amounts_hm.insert(1, 0);
            bucket_amounts_hm.insert(2, capacity_2);
        }
    }
    let mut moves: u8 = 1;
    println!("start_bucket: {}", start_bucket);
    loop {
        moves += 1;
        println!("move #{}", moves);
        pour(
            &mut bucket_amounts_hm,
            capacity_1,
            capacity_2,
            start_bucket,
        );
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
    let (bucket_one_amount, bucket_two_amount) = get_bucket_amounts(bucket_amounts_hm);

    let bucket_one_space = bucket_one_capacity - bucket_one_amount;
    let bucket_two_space = bucket_two_capacity - bucket_two_amount;

    let mut new_bucket_one_amount: u8 = 0; 
    let mut new_bucket_two_amount: u8 = 0;

    if bucket_one_amount == bucket_one_capacity { 
        match bucket_one_amount.cmp(&bucket_two_space) {
            Ordering::Less => {
                let poured_amount = bucket_one_amount;
                new_bucket_one_amount = 0;
                new_bucket_two_amount = bucket_two_amount + poured_amount;
            },
            Ordering::Equal => {
                new_bucket_one_amount = 0;
                new_bucket_two_amount = bucket_two_amount;
            },
            Ordering::Greater => {
                let poured_amount = bucket_two_space;
                new_bucket_one_amount = bucket_one_amount - poured_amount;
                new_bucket_two_amount = bucket_two_capacity;
            }
        }
    } else if bucket_two_amount == bucket_two_capacity {
        match bucket_two_amount.cmp(&bucket_one_space) {
            Ordering::Less => {
                let poured_amount = bucket_two_amount;
                new_bucket_one_amount = bucket_one_amount + poured_amount;
                new_bucket_two_amount = 0;
            },
            Ordering::Equal => {
                new_bucket_one_amount = bucket_one_amount;
                new_bucket_two_amount = 0;
            },
            Ordering::Greater => {
                let poured_amount = bucket_one_space;
                new_bucket_one_amount = bucket_one_capacity;
                new_bucket_two_amount = bucket_two_amount - poured_amount;
            }
        }
    } else {
        // TEST 2 (3, 5, 1, &Bucket::Two) -> 8
        //   (0, 5)
        //   (3, 2) -> bucket 1 + bucket 2 = bucket 2 capacity
        //   (0, 2)
        //   (2, 0)
        //   (2, 5)
        //   (3, 4)
        //   (0, 4)
        //   (3, 1)
        match *start_bucket {
            Bucket::One => {
                if bucket_two_amount == 0 {
                    let poured_amount = min(bucket_two_space, bucket_one_amount);
                    new_bucket_one_amount = bucket_one_amount - poured_amount;
                    new_bucket_two_amount = poured_amount;
                } else if bucket_one_amount == 0 {
                    new_bucket_one_amount = bucket_one_capacity;
                    new_bucket_two_amount = bucket_two_amount;
                }
            },
            Bucket::Two => {
                if bucket_one_amount == 0 {
                    let poured_amount = min(bucket_one_space, bucket_two_amount);
                    new_bucket_one_amount = poured_amount;
                    new_bucket_two_amount = bucket_two_amount - poured_amount;
                } else if bucket_two_amount == 0 {
                    new_bucket_one_amount = bucket_one_amount;
                    new_bucket_two_amount = bucket_two_capacity;
                }
            }
        }
    }
    println!("({}, {})", new_bucket_one_amount, new_bucket_two_amount);
    bucket_amounts_hm.insert(1, new_bucket_one_amount);
    bucket_amounts_hm.insert(2, new_bucket_two_amount);

    // println!("pour is called, pouring into: {}", receiving_bucket);
    // let (mut pouring_bucket_amount, mut receiving_bucket_amount, receiving_bucket_capacity) =
    //     match receiving_bucket {
    //         Bucket::One => (bucket_two_amount, bucket_one_amount, bucket_one_capacity),
    //         Bucket::Two => (bucket_one_amount, bucket_two_amount, bucket_two_capacity),
    //     };

    // let (new_bucket_one_amount, new_bucket_two_amount) = match receiving_bucket {
    //     receiving_bucket @ Bucket::One => {
    //         let new_bucket_one_amount;
    //         let new_bucket_two_amount;

    //         if start_bucket == receiving_bucket && bucket_one_amount == 0 {
    //             new_bucket_one_amount = bucket_one_capacity;
    //             new_bucket_two_amount = bucket_two_amount;
    //         } else if bucket_one_amount + bucket_two_amount > bucket_one_capacity {
    //             let poured_amount = bucket_one_capacity - bucket_one_amount;
    //             new_bucket_one_amount = bucket_one_amount + poured_amount;
    //             new_bucket_two_amount = bucket_two_amount - poured_amount;
    //         } else {
    //             new_bucket_one_amount = bucket_one_amount + bucket_two_amount;
    //             new_bucket_two_amount = 0;
    //         }
    //         (new_bucket_one_amount, new_bucket_two_amount)
    //     }
    // };

    // if pouring into one bucket, but doing nothing with the other 
    // if receiving_bucket_amount == 0 {
    //     receiving_bucket_amount = receiving_bucket_capacity;

    // // if pouring from one bucket into another
    // } else if pouring_bucket_amount + receiving_bucket_amount > receiving_bucket_capacity {
    //     let poured_amount = receiving_bucket_capacity - receiving_bucket_amount;
    //     pouring_bucket_amount -= poured_amount;
    //     receiving_bucket_amount += poured_amount;
    // // if emptying one bucket
    // // } else if {

    // }

    
    // let (new_bucket_one_amount, new_bucket_two_amount) = match current_move % 2 == 1 {
    //     // first turn, then every other turn
    //     true => match *start_bucket {
    //         Bucket::One => (receiving_bucket_amount, pouring_bucket_amount),
    //         Bucket::Two => (pouring_bucket_amount, receiving_bucket_amount),
    //     },
    //     // second turn, then every other turn
    //     false => match *start_bucket {
    //         Bucket::One => (pouring_bucket_amount, receiving_bucket_amount),
    //         Bucket::Two => (receiving_bucket_amount, pouring_bucket_amount),
    //     },
    // };

    // bucket_amounts_hm.insert(1, new_bucket_one_amount);
    // bucket_amounts_hm.insert(2, new_bucket_two_amount);
    // println!("move #{}: ", current_move);
    // println!("({}, {})", bucket_amounts_hm[&1], bucket_amounts_hm[&2]);
}

fn check_solved(bucket_amounts_hm: &HashMap<usize, u8>, goal: u8) -> bool {
    let (bucket_one_amount, bucket_two_amount) = get_bucket_amounts(bucket_amounts_hm);
    bucket_one_amount == goal || bucket_two_amount == goal || (bucket_one_amount == 0 && bucket_two_amount == 0)
}

fn get_result_stats(
    moves: u8,
    bucket_amounts_hm: &HashMap<usize, u8>,
    goal: u8,
) -> Option<BucketStats> {
    let (bucket_one_amount, bucket_two_amount) = get_bucket_amounts(bucket_amounts_hm);
    if (0, 0) == (bucket_one_amount, bucket_two_amount) {
        return None;
    }
    let bucket_amounts = (bucket_one_amount, bucket_two_amount);

    // TODO: read from goal, not using literal 1
    let (goal_bucket, other_bucket) = match bucket_amounts {
        (1, _) => (Bucket::One, bucket_amounts.1),
        (_, 1) => (Bucket::Two, bucket_amounts.0),
        _ => {
            println!("get_result_stats is called, here are the args:\nmoves: {}\nbucket_amounts: ({}, {})\n goal: {}", moves, bucket_amounts.0, bucket_amounts.1, goal);
            return None;
        }
    };
    Some(BucketStats {
        moves,
        goal_bucket,
        other_bucket,
    })
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
