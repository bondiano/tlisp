use crate::object::Object;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
}
