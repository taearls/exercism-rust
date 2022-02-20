pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    let mut is_even = true;
    iter.filter(move |_| {
        is_even = !is_even;
        !is_even 
    })
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        let (x, y) = (self.0, self.1);
        x.abs() + y.abs()
    }
}
