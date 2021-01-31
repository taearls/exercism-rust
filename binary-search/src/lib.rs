use std::cmp::Ordering;

pub fn find<T, U>(array: T, key: U) -> Option<usize>
where
    T: AsRef<[U]>,
    U: Ord,
{
    let array_ref = array.as_ref();
    let mut left = 0;
    let mut right = array_ref.len();

    // only perform search if len is greater than 0
    if right > 0 {
        while left < right {
            let mid = (left + right) / 2;
            let val = &array_ref[mid];
            match key.cmp(val) {
                Ordering::Greater => left = mid + 1,
                Ordering::Less => right = mid,
                Ordering::Equal => return Some(mid),
            }
        }
    }
    None
}
