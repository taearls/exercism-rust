use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}
// Test Case 1

// Move 1:
// Fill Bucket 1 (3, 0)


// solve(3, 5, 1, &Bucket::One),
//         Some(BucketStats {
//             moves: 4,
//             goal_bucket: Bucket::One,
//             other_bucket: 5,
//         })

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut bucket_amounts = (0, 0);
    let mut moves: u8 = 0;

    // perform first move
    let poured_amount = match *start_bucket {
        Bucket::One => capacity_1,
        Bucket::Two => capacity_2,
    };
    bucket_amounts = pour(start_bucket, poured_amount, &bucket_amounts);
    moves += 1;
    println!("bucket_amounts after first move: ({}, {})", bucket_amounts.0, bucket_amounts.1);
    // perform subsequent moves, if necessary
    loop {
        if check_solved(bucket_amounts, goal) { 
            break;
        }
        moves += 1;

        // TODO: figure out moves in these scenarios

        // do I need to match against each bucket's capacity as well?
        let (pouring_bucket, poured_amount) = match bucket_amounts.0.cmp(&bucket_amounts.1) {
            Ordering::Greater => (&Bucket::One, capacity_2),
            Ordering::Less => (&Bucket::Two, capacity_1),
            Ordering::Equal => break,
        };
        pour(pouring_bucket, poured_amount, &bucket_amounts);
    }
    
    let result = get_result_stats(moves, bucket_amounts, goal);
    result
}

fn pour(pouring_bucket: &Bucket, poured_amount: u8, mut bucket_amounts: &(u8, u8)) -> (u8, u8) {
    println!("pour is called, here are the args:\n poured_amount: {}\n bucket_amounts: ({}, {})", poured_amount, bucket_amounts.0, bucket_amounts.1);
    let mut new_bucket_amounts = (bucket_amounts.0, bucket_amounts.1);

    match *pouring_bucket {
        Bucket::One => {
            if bucket_amounts.0 > poured_amount {
                new_bucket_amounts.0 -= poured_amount;
            }
            new_bucket_amounts = match new_bucket_amounts.0.cmp(&poured_amount) {
                Ordering::Greater => {
                    new_bucket_amounts.0 -= poured_amount;
                    new_bucket_amounts.1 += poured_amount;
                    new_bucket_amounts
                },
                Ordering::Less => {
                    if new_bucket_amounts.1 > poured_amount {
                        new_bucket_amounts.1 -= poured_amount;
                        new_bucket_amounts.0 += poured_amount;
                    }
                    new_bucket_amounts
                },
                Ordering::Equal => new_bucket_amounts
            }
        },
        Bucket::Two => {
            if new_bucket_amounts.1 > poured_amount {
             new_bucket_amounts.1 -= poured_amount;
                new_bucket_amounts.0 += poured_amount;
            }
            new_bucket_amounts = match new_bucket_amounts.1.cmp(&poured_amount) {
                Ordering::Greater => {
                    new_bucket_amounts.1 -= poured_amount;
                    new_bucket_amounts
                },
                Ordering::Less => {
                    if new_bucket_amounts.0 > poured_amount {
                        new_bucket_amounts.0 -= poured_amount;
                        new_bucket_amounts.1 += poured_amount;
                    }
                    new_bucket_amounts
                },
                Ordering::Equal => new_bucket_amounts
            }
        }
    }
    new_bucket_amounts
}

fn check_solved(bucket_amounts: (u8, u8), goal: u8) -> bool {
    println!("check_solved is called, here are the args:\n bucket_amounts: ({}, {})\n goal: {}", bucket_amounts.0, bucket_amounts.1, goal);
    println!("result of check_solved: {}", bucket_amounts.0 == goal || bucket_amounts.1 == goal);
    bucket_amounts.0 == goal || bucket_amounts.1 == goal
}

fn get_result_stats(moves: u8, bucket_amounts: (u8, u8), goal: u8) -> Option<BucketStats> {
    let (goal_bucket, other_bucket) = match bucket_amounts {
        (1, _) => (Bucket::One, bucket_amounts.1),
        (_, 1) => (Bucket::Two, bucket_amounts.0),
        _ => {
            println!("get_result_stats is called, here are the args:\nmoves: {}\nbucket_amounts: ({}, {})\n goal: {}", moves, bucket_amounts.0, bucket_amounts.1, goal);
            return None;
        },
    };
    Some(BucketStats {
        moves,
        goal_bucket,
        other_bucket
    })
}