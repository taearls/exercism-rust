use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
  pub name: String,
  attrs: HashMap<String, String>,
}

impl Node {
  pub fn new(name: &str) -> Self {
    Node {
      name: name.to_string(),
      attrs: HashMap::new(),
    }
  }
  pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
    for (key, value) in attrs.iter() {
      self.attrs.insert(key.to_string(), value.to_string());
    }
    self
  }
  pub fn get_attr(&self, attr_name: &str) -> Option<&str> {
    match &self.attrs.get(attr_name) {
      Some(s) => return Some(s),
      None => return None,
    }
  }
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

impl fmt::Display for Node {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Node {{ name: {} }})", self.name)
  }
}