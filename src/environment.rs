use crate::object::Object;
use std::{cell::RefCell, collections::HashMap, rc::Rc, fmt};

#[derive(Debug, PartialEq, Default)]
pub struct Environment {
  parent: Option<Rc<RefCell<Environment>>>,
  vars: HashMap<String, Object>,
}

impl Environment {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn extend(parent: Rc<RefCell<Self>>) -> Environment {
    Environment {
      vars: HashMap::new(),
      parent: Some(parent),
    }
  }

  pub fn get(&self, name: &str) -> Option<Object> {
    match self.vars.get(name) {
      Some(value) => Some(value.clone()),
      None => self.parent.as_ref().and_then(|o| o.borrow().get(name)),
    }
  }

  pub fn set(&mut self, name: &str, val: Object) {
    self.vars.insert(name.to_string(), val);
  }

  pub fn update(&mut self, data: Rc<RefCell<Self>>) {
    self.vars.extend(
      data
        .borrow()
        .vars
        .iter()
        .map(|(k, v)| (k.clone(), v.clone())),
    );
  }
}

impl fmt::Display for Environment {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut vars_str = String::new();

    for (k, v) in self.vars.iter() {
      vars_str.push_str(&format!("{}: {}\n", k, v));
    }

    let _ = match self.parent {
      Some(ref parent) => {
        vars_str.push_str(&format!("parent: {}\n", parent.borrow()));
      },
      None => {
        vars_str.push_str("parent: None\n");
      },
    };

    write!(f, "{}", vars_str)
  }
}
