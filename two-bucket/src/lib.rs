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
    // set default values for bucket 1 and bucket 2
    bucket_amounts_hm.insert(1, 0);
    bucket_amounts_hm.insert(2, 0);
    println!(
        "bucket one capacity: {}, bucket two capacity: {}",
        capacity_1, capacity_2
    );

    let mut receiving_bucket = match *start_bucket {
        Bucket::One => Bucket::One,
        Bucket::Two => Bucket::Two,
    };
    let mut moves: u8 = 0;
    loop {
        moves += 1;
        pour(
            &start_bucket,
            &receiving_bucket,
            &mut bucket_amounts_hm,
            capacity_1,
            capacity_2,
            moves,
        );
        receiving_bucket = get_receiving_bucket(&receiving_bucket);
        if check_solved(&bucket_amounts_hm, goal) {
            break;
        }
    }

    get_result_stats(moves, &bucket_amounts_hm, goal)
}

fn pour(
    start_bucket: &Bucket,
    receiving_bucket: &Bucket,
    bucket_amounts_hm: &mut HashMap<usize, u8>,
    bucket_one_capacity: u8,
    bucket_two_capacity: u8,
    current_move: u8,
) {
    let (bucket_one_amount, bucket_two_amount) = get_bucket_amounts(bucket_amounts_hm);

    println!("pour is called, pouring into: {}", receiving_bucket);
    let (mut pouring_bucket_amount, mut receiving_bucket_amount, receiving_bucket_capacity) =
        match receiving_bucket {
            Bucket::One => (bucket_two_amount, bucket_one_amount, bucket_one_capacity),
            Bucket::Two => (bucket_one_amount, bucket_two_amount, bucket_two_capacity),
        };

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

    // if initial pour
    if receiving_bucket_amount == 0 {
        receiving_bucket_amount = receiving_bucket_capacity;
    } else if bucket_one_amount + bucket_two_amount > receiving_bucket_capacity {
        let poured_amount = receiving_bucket_capacity - receiving_bucket_amount;
        pouring_bucket_amount -= poured_amount;
        receiving_bucket_amount += poured_amount;
    }
    let (new_bucket_one_amount, new_bucket_two_amount) = match current_move % 2 == 1 {
        true => match *start_bucket {
            Bucket::One => (pouring_bucket_amount, receiving_bucket_amount),
            Bucket::Two => (pouring_bucket_amount, receiving_bucket_amount),
        },
        false => match *start_bucket {
            Bucket::One => (pouring_bucket_amount, receiving_bucket_amount),
            Bucket::Two => (receiving_bucket_amount, pouring_bucket_amount),
        },
    };

    bucket_amounts_hm.insert(1, new_bucket_one_amount);
    bucket_amounts_hm.insert(2, new_bucket_two_amount);
    println!("move #{}: ", current_move);
    println!("({}, {})", bucket_amounts_hm[&1], bucket_amounts_hm[&2]);
}

fn check_solved(bucket_amounts_hm: &HashMap<usize, u8>, goal: u8) -> bool {
    let (bucket_one_amount, bucket_two_amount) = get_bucket_amounts(bucket_amounts_hm);
    bucket_one_amount == goal || bucket_two_amount == goal
}

fn get_result_stats(
    moves: u8,
    bucket_amounts_hm: &HashMap<usize, u8>,
    goal: u8,
) -> Option<BucketStats> {
    let (bucket_one_amount, bucket_two_amount) = get_bucket_amounts(bucket_amounts_hm);
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

fn get_receiving_bucket(bucket: &Bucket) -> Bucket {
    match *bucket {
        Bucket::One => Bucket::Two,
        Bucket::Two => Bucket::One,
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
