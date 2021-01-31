pub fn find<T, U> (array: T, key: U) -> Option<usize>
where 
    T: AsRef<[U]>,
    U: Ord, 
{
    let array_ref = array.as_ref();
    let mut left = 0;
    let mut right = array_ref.len();
    // if len is zero, subtracting 1 will create an overflow error bc it's a usize
    if right == 0 {
        return None;
    }
    loop {
        if left > right {
            break;
        }

        let mid = (left + right) / 2;
        let val = &array_ref[mid];
        if *val < key {
            if mid == array_ref.len() - 1 {
                break;
            } else {
                left = mid + 1;
            }
        } else if *val > key {
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
    None
}
