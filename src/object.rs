use std::{cell::RefCell, fmt, rc::Rc};

use crate::environment::Environment;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
  Void,
  If,
  Keyword(String),
  BinaryOp(String),
  Float(f64),
  Integer(i64),
  Bool(bool),
  String(String),
  Symbol(String),
  Lambda(Vec<String>, Vec<Object>, Rc<RefCell<Environment>>),
  List(Vec<Object>),
}

impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Object::Void => write!(f, "Void"),
      Object::Integer(n) => write!(f, "{}", n),
      Object::Bool(b) => write!(f, "{}", b),
      Object::Symbol(s) => write!(f, "{}", s),
      Object::Lambda(params, body, _env) => {
        write!(f, "Lambda(")?;
        for param in params {
          write!(f, "{} ", param)?;
        }
        write!(f, ")")?;
        for expr in (*body).iter() {
          write!(f, " {}", expr)?;
        }
        Ok(())
      }
      Object::List(list) => {
        write!(f, "(")?;
        for (i, obj) in (*list).iter().enumerate() {
          if i > 0 {
            write!(f, " ")?;
          }
          write!(f, "{}", obj)?;
        }
        write!(f, ")")
      }
      Object::Keyword(s) => write!(f, "{}", s),
      Object::BinaryOp(s) => write!(f, "{}", s),
      Object::Float(n) => write!(f, "{}", n),
      Object::String(s) => write!(f, "{}", s),
      Object::If => write!(f, "if"),
    }
  }
}
