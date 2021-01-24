pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 {
        return None
    }

    let mut steps = 0;
    let mut temp_n = n;
    loop {
        if temp_n == 1 {
            break;
        } else if temp_n % 2 == 1 {
            temp_n = (3 * temp_n) + 1;
        } else {
            temp_n /= 2;
        }
        steps += 1;
    }

    Some(steps)
}
