pub fn verse(n: u32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        _ => standard_verse(n)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut str = String::new();
    for i in (end..=start).rev() {
        str.push_str(&verse(i));
        if i != end {
            str.push_str("\n")
        }
    }
    str
}

fn standard_verse(n: u32) -> String {                                           
    let next = n - 1;                                                           
    let next_bottle = if next == 1 { "bottle" } else { "bottles" };    
    // format! macro makes it easy to concatenate different types in a string slice         
    format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} {2} of beer on the wall.\n", n, next, next_bottle)
}    