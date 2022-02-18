#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    for (index, first_list_item) in first_list.iter().enumerate() {
        match second_list.get(index) {
            None => return Comparison::Superlist,
            Some(second_list_item) => {
                if first_list_item != second_list_item {
                    return Comparison::Unequal;
                }
            }
        }
    }
    if first_list.len() != second_list.len() {
        Comparison::Sublist
    } else {
        Comparison::Equal
    }
}
