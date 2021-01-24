pub fn factors(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    let mut num = n;
    let mut i = 3;

    // check for 2 first, then only check odd numbers
    loop {
        if num % 2 == 0 {
            primes.push(2);
            num /= 2;
        } else {
            break;
        }
    }

    loop {
        if num % i == 0 {
            primes.push(i);
            num /= i;
        } else {
            i += 2;
        }
        if num == 1 {
            break;
        }
    }
    primes
}