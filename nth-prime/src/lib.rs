pub fn nth(n: u32) -> u32 {
    let mut vec: Vec<u32> = vec![2, 3];
    let mut max = 5;
    let n_usize = n as usize;

    // start with primes greater than the 2nd so
    // we can have a sqrt greater than 2 to check
    if n > 1 {
        loop {
            if is_prime(max) {
                vec.push(max)
            }
            if vec.len() - 1 == n_usize {
                break;
            }
            max += 1;
        }
    }
    vec[n_usize]
}

fn is_prime(num: u32) -> bool {
    for i in 2..=get_max(num) {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

fn get_max(num: u32) -> u32 {
    (num as f64)
        .sqrt()
        .floor() as u32
}