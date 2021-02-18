use std::fmt; 
use std::collections::HashMap;

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
    // perform first move
    let mut bucket_amounts_hm: HashMap<usize, u8> = HashMap::new();

    match *start_bucket {
        Bucket::One => {
            bucket_amounts_hm.insert(1, capacity_1);
            bucket_amounts_hm.insert(2, 0);
        },
        Bucket::Two => {
            bucket_amounts_hm.insert(1, 0);
            bucket_amounts_hm.insert(2, capacity_2);
        },
    };

    let mut pouring_bucket = get_pouring_bucket(start_bucket);

    let mut moves: u8 = 1;
    // first move is working.
    println!("bucket_amounts after move #1: ({}, {})", bucket_amounts_hm.get(&1).unwrap(), bucket_amounts_hm.get(&2).unwrap());

    // perform subsequent moves, if necessary

    loop {
        if check_solved(&bucket_amounts_hm, goal) { 
            break;
        }
        // TODO: figure out moves in these scenarios

        // do I need to match against each bucket's capacity as well?

        // should compare Bucket One to Bucket Two. implement partial eq with value for each bucket being the amount assigned to it. instead of using bucket_amounts var, write value to enum?
        // let (pouring_bucket, poured_amount) = match bucket_one_amount.cmp(&bucket_two_amount) {
        //     Ordering::Greater => {
        //         // TODO: don't pour capacity. pour amount in bucket.
        //         // only pour if there's enough room in other bucket.
        //         (&Bucket::One, capacity_1)
        //     },
        //     Ordering::Less => (&Bucket::Two, capacity_1),
        //     Ordering::Equal => break,
        // };
        moves = pour(&start_bucket, &pouring_bucket, &mut bucket_amounts_hm, capacity_1, capacity_2, moves);
        pouring_bucket = get_pouring_bucket(&pouring_bucket);
    }
    
    let result = get_result_stats(moves, &bucket_amounts_hm, goal);
    result
}

// Move 1:
// Fill Bucket 1 (3, 0)

// Move 2: 
// Fill Bucket 2 (0, 3)

// Move 3:
// Fill Bucket 1 (3, 3)

// Move 4:
// Fill Bucket 2 (1, 5)
fn pour(start_bucket: &Bucket, pouring_bucket: &Bucket, bucket_amounts_hm: &mut HashMap<usize, u8>, bucket_one_capacity: u8, bucket_two_capacity: u8, moves: u8) -> u8 {
    let bucket_one_amount = bucket_amounts_hm[&1];  
    let bucket_two_amount = bucket_amounts_hm[&2];
    println!("pour is called, pouring from: {}", pouring_bucket);
    let (new_bucket_one_amount, new_bucket_two_amount) = match pouring_bucket {
        bucket @ Bucket::One => {
            let new_bucket_one_amount;
            let new_bucket_two_amount;

            if start_bucket == bucket && bucket_one_amount == 0 {
                new_bucket_one_amount = bucket_one_capacity;
                new_bucket_two_amount = bucket_two_amount;
            } else if bucket_one_amount + bucket_two_amount > bucket_two_capacity {
                if bucket_one_capacity > bucket_two_capacity { return moves; }

                let poured_amount = bucket_two_capacity - bucket_two_amount;
                new_bucket_one_amount = bucket_one_amount - poured_amount;
                new_bucket_two_amount = bucket_two_amount + poured_amount;
            } else {
                new_bucket_one_amount = bucket_one_amount + bucket_two_amount;
                new_bucket_two_amount = 0;
            }
            (new_bucket_one_amount, new_bucket_two_amount)
        },
        bucket @ Bucket::Two => {
            let new_bucket_one_amount;
            let new_bucket_two_amount;

            if start_bucket == bucket && bucket_two_amount == 0 {
                new_bucket_one_amount = bucket_one_amount;
                new_bucket_two_amount = bucket_two_capacity;
            } else if bucket_one_amount + bucket_two_amount > bucket_one_capacity {
                if bucket_two_capacity > bucket_one_capacity { return moves; }
                let poured_amount = bucket_one_capacity - bucket_one_amount;
                new_bucket_one_amount = bucket_one_amount + poured_amount;
                new_bucket_two_amount = bucket_two_amount - poured_amount;
            } else {
                new_bucket_one_amount = 0;
                new_bucket_two_amount = bucket_one_amount + bucket_two_amount;
            }
            // println!("new_bucket_one_amount: {}, new_bucket_two_amount: {}", new_bucket_one_amount, new_bucket_two_amount);
            (new_bucket_one_amount, new_bucket_two_amount)
        }
        // Move 1:
        // Fill Bucket 1 (3, 0)

        // Move 2: 
        // Fill Bucket 2 (0, 3)

        // Move 3:
        // Fill Bucket 1 (3, 3)

        // Move 4:
        // Fill Bucket 2 (1, 5)
    };
    bucket_amounts_hm.insert(1, new_bucket_one_amount);
    bucket_amounts_hm.insert(2, new_bucket_two_amount);
    println!("move #{}: ", moves + 1);
    println!("({}, {})", bucket_amounts_hm[&1], bucket_amounts_hm[&2]);
    moves + 1
}

fn check_solved(bucket_amounts_hm: &HashMap<usize, u8>, goal: u8) -> bool {
    let bucket_one_amount = *bucket_amounts_hm.get(&1).unwrap();
    let bucket_two_amount = *bucket_amounts_hm.get(&2).unwrap();
    
    bucket_one_amount == goal || bucket_two_amount == goal
}

fn get_result_stats(moves: u8, bucket_amounts_hm: &HashMap<usize, u8>, goal: u8) -> Option<BucketStats> {
    let bucket_one_amount = *bucket_amounts_hm.get(&1).unwrap();
    let bucket_two_amount = *bucket_amounts_hm.get(&2).unwrap();
    let bucket_amounts = (bucket_one_amount, bucket_two_amount);

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

fn get_pouring_bucket(bucket: &Bucket) -> Bucket {
    match *bucket {
        Bucket::One => Bucket::Two,
        Bucket::Two => Bucket::One
    }
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