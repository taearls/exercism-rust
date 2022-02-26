#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T> CustomSet<T>
where T: Copy + PartialEq
{
    pub fn new(_input: &[T]) -> Self {
        let mut data: Vec<T> = _input.to_vec();
        for &val in _input.into_iter() {
            data.push(val);
        }
        Self {
            data,
        }
    }

    pub fn contains(&self, _element: &T) -> bool {
       self.data.iter().filter(|&x| x == _element).count() > 0
    }

    pub fn add(&mut self, _element: T) {
        if !self.contains(&_element) {
            self.data.push(_element)
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        unimplemented!();
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        unimplemented!();
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        unimplemented!();
    }
}
