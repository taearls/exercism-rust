#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T> CustomSet<T>
where
    T: Copy + PartialEq + Ord,
{
    pub fn new(_input: &[T]) -> Self {
        let mut data: Vec<T> = _input.to_vec();
        for val in _input.iter() {
            if !data.contains(val) {
                data.push(*val);
            }
        }
        data.sort();
        Self { data }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.data.iter().filter(|&x| x == _element).count() > 0
    }

    pub fn add(&mut self, _element: T) {
        if !self.contains(&_element) {
            self.data.push(_element);
            self.data.sort();
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        if self.data.len() > _other.data.len() {
            return false;
        } else if self.is_empty() && _other.is_empty() {
            return true;
        }
        let mut result = true;
        for val in self.data.iter() {
            if !_other.contains(val) {
                result = false;
                break;
            }
        }
        result
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        let mut result = true;
        for val in self.data.iter() {
            if _other.contains(val) {
                result = false;
                break;
            }
        }
        result
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        let data = self
            .data
            .iter()
            .filter(|x| _other.contains(x))
            .copied()
            .collect::<Vec<T>>();
        Self { data }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let mut data = self
            .data
            .iter()
            .filter(|x| !_other.contains(x))
            .copied()
            .collect::<Vec<T>>();
        let mut other_data = _other
            .data
            .iter()
            .filter(|x| !_other.contains(x))
            .copied()
            .collect::<Vec<T>>();
        data.append(&mut other_data);
        data.sort();
        Self { data }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let mut data = self.data.to_vec();
        for val in _other.data.iter() {
            if !data.contains(val) {
                data.push(*val);
            }
        }
        data.sort();
        Self { data }
    }
}
