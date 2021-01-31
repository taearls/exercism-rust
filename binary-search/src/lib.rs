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
    while left < right {
        let mid = (left + right) / 2;
        let val = &array_ref[mid];
        if key > *val {
            // if the target value is greater, we only need to search to the right
            left = mid + 1;
        } else if key < *val {
            right = mid;
        } else {
            return Some(mid);
        }
    }
    None
}
