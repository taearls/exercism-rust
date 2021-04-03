/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        None
    } else {
        match s1.len() {
            0 => Some(0),
            _ => {
                let mut distance: usize = 0;
                for (i, c) in s1.char_indices() {
                    if c != s2.chars().nth(i).unwrap() {
                        distance += 1;
                    }
                }
                Some(distance)
            }
        }
    }
}
