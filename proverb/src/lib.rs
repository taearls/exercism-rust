pub fn build_proverb(list: &[&str]) -> String {
    let mut str = String::new();
    
    // start with length of list, decrement by 1 for each iteration of loop
    let mut items_left = list.len();
    
    // if there's more than one item, print the next line using the current and next item
    if items_left > 1 {
        loop {
            let index = list.len() - items_left;
            str.push_str(&format!("For want of a {item} the {next_item} was lost.\n", item = list.get(index).unwrap(), next_item = list.get(index + 1).unwrap()));
            items_left -= 1;
            if items_left == 1 {
                break;
            }
        }
    }
    // if there's one item left in the list, print the last line
    if items_left == 1 {
        str.push_str(&format!("And all for the want of a {0}.", list.get(0).unwrap()));
    }
    str
}
