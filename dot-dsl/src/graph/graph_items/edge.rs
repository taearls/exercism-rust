use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Edge {
  a: String,
  b: String,
  attrs: HashMap<String, String>,
}

impl Edge { 
  pub fn new(a: &str, b: &str) -> Self {
    Edge {
      a: a.to_string(),
      b: b.to_string(),
      attrs: HashMap::new(),
    }
  }
  pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
    for (key, value) in attrs.iter() {
      self.attrs.insert(key.to_string(), value.to_string());
    }
    self
  }
}

impl PartialEq for Edge {
  fn eq(&self, other: &Self) -> bool {
    self.a == other.a && self.b == other.b
  }
}