pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // if len is zero, subtracting 1 will create an overflow error bc it's a usize
    if array.len() == 0 {
        return None;
    }
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
                    // if mid is 0 and the number is greater than the target key, 
                    // the target is smaller than the smallest element in the array and we should return None
                    if mid == 0 { 
                        break; 
                    } else {
                        right = mid - 1;
                    }
                } else {
                    return Some(mid);
                }
            }
            None => break,
        }
    }
    None
}
