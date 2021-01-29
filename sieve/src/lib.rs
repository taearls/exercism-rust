// Sieve of Eratosthenes algorithm found here: 
// https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut prime_nums: Vec<u64> = Vec::new();
    if upper_bound > 1 {
        let mut composite_nums: Vec<u64> = Vec::new();

        for n in 2..=upper_bound {
            // if it's not in the composite nums vec, it's a prime.
            // use sieve algorithm to mark all multiples as composite.
            if !composite_nums.contains(&n) {
                prime_nums.push(n);

                let mut mult = 1;
                loop {
                    let value = mult * n;
                    if value > upper_bound {
                        break;
                    }
                    composite_nums.push(value);
                    mult += 1;
                }
            }
        }
    }
    prime_nums
}
