pub fn map<T, F, U>(input: Vec<T>, mut _function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut vec: Vec<U> = Vec::with_capacity(input.len());
    for item in input {
        vec.push(_function(item));
    }
    vec
}
