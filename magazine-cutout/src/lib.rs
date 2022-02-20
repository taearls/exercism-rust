use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut hm: HashMap<&str, usize> = HashMap::new();

    for word in magazine.iter() {
        *hm.entry(word).or_insert(0) += 1;
    }
    for word in note.iter() {
        if hm.contains_key(word) && *hm.get(word).unwrap() > 0 {
            *hm.get_mut(word).unwrap() -= 1;
        } else {
            return false;
        }
    }
    true
}
