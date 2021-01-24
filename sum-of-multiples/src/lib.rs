pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum: u32 = 0;
    for i in 1..limit {
        let mut added = false;
        for j in factors {
            if *j == 0 {
                continue;
            }
            if i % j == 0 && !added {
                added = true;
                sum += i;
            }
        }
    }
    sum
}
