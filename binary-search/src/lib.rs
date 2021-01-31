pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len() - 1;
    loop {
        if left > right {
            break;
        }

        let mid = (((left + right) as f32) / 2.0).floor() as usize;
        match array.get(mid) {
            Some(num) => {
                if *num < key {
                    left = mid + 1;
                } else if *num > key {
                    right = mid - 1;
                } else {
                    return Some(mid);
                }
            }
            None => break,
        }
    }
    None
}
