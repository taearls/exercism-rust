use std::ops::Rem;

pub struct Matcher<T> {
    matcher_fn: fn(T) -> bool,
    sub: String,
}

impl<T> Matcher<T> {
    pub fn new<S: ToString>(_matcher: fn(T) -> bool, _subs: S) -> Matcher<T> {
        Matcher {
            matcher_fn: _matcher,
            sub: _subs.to_string(),
        }
    }
}

pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T> {
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.matchers.push(_matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item = T>>(self, _iter: I) -> impl Iterator<Item = String>
    where
        T: Copy + ToString + PartialEq,
    {
        _iter.map(move |x| {
            let mut result = x.to_string();
            for matcher in self.matchers.iter() {
                let matcher_fn = matcher.matcher_fn;
                if matcher_fn(x) {
                    result = matcher.sub.clone()
                }
            }
            result
        })
    }
}
impl<T> Default for Fizzy<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: Rem<Output = T> + From<u8> + Copy + PartialEq>() -> Fizzy<T> {
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3_u8.into() == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5_u8.into() == T::from(0), "buzz"))
        .add_matcher(Matcher::new(
            |n: T| n % 15_u8.into() == T::from(0),
            "fizzbuzz",
        ))
}
