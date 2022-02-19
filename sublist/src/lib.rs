use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    // determine the shorter and longer list, accounting for cases where one or both lists are empty.
    let (shorter_list, longer_list): (&[T], &[T]) = match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal => {
            if first_list.is_empty() && second_list.is_empty() {
                return Comparison::Equal;
            } else {
                (first_list, second_list)
            }
        }
        Ordering::Less => {
            if first_list.is_empty() {
                return Comparison::Sublist;
            } else {
                (first_list, second_list)
            }
        }
        Ordering::Greater => {
            if second_list.is_empty() {
                return Comparison::Superlist;
            } else {
                (second_list, first_list)
            }
        }
    };

    // iterate through the length of the shorter list, comparing slices of that length from the longer list
    let slice_len = shorter_list.len();

    let mut match_found = false;
    for index in 0..shorter_list.len() {
        match longer_list.get(index..(index + slice_len)) {
            None => break,
            Some(longer_list_slice) => {
                if longer_list_slice == shorter_list {
                    match_found = true;
                    break;
                }
            }
        }
    }

    if !match_found {
        Comparison::Unequal
    } else {
        match first_list.len().cmp(&second_list.len()) {
            Ordering::Equal => Comparison::Equal,
            Ordering::Less => Comparison::Sublist,
            Ordering::Greater => Comparison::Superlist,
        }
    }
}
