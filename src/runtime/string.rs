use crate::{environment::Environment, object::Object};
use dyn_fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{list::unquote, RuntimeFn};

fn format_(args: &Vec<Object>, _env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let arg = args.get(0).unwrap();

  let rest = args.get(1..).unwrap();

  match arg {
    Object::String(str) => Ok(Object::String(
      dyn_fmt::AsStrFormatExt::format(str, rest).to_string(),
    )),
    _ => Ok(Object::Void),
  }
}

fn split(args: &Vec<Object>, _env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let str = args.get(0);

  let str = match str {
    Some(Object::String(s)) => s,
    _ => return Ok(Object::Void),
  };

  let separator = args.get(1);

  let separator = match separator {
    Some(Object::String(s)) => s.clone(),
    _ => "".to_string(),
  };

  let result = match separator.as_str() {
    "" => str
      .split("")
      .filter(|&x| !x.is_empty())
      .map(|s| Object::String(s.to_string()))
      .collect::<Vec<Object>>(),
    " " => str
      .split_whitespace()
      .map(|s| Object::String(s.to_string()))
      .collect::<Vec<Object>>(),
    _ => str
      .split(&separator)
      .map(|s| Object::String(s.to_string()))
      .collect::<Vec<Object>>(),
  };

  Ok(Object::List(result))
}

fn join(args: &Vec<Object>, _env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let list = unquote(args);
  let list = match list {
    Object::List(list) => list,
    _ => return Ok(Object::Void),
  };

  let separator = args.get(1);
  let separator = match separator {
    Some(Object::String(s)) => s.clone(),
    _ => "".to_string(),
  };

  let result = list
    .iter()
    .map(|o| o.to_string())
    .collect::<Vec<String>>()
    .join(&separator);

  Ok(Object::String(result))
}

pub fn load_string_fns(methods: &mut HashMap<String, Rc<RuntimeFn>>) {
  methods.insert("format".to_string(), Rc::new(format_));
  methods.insert("split".to_string(), Rc::new(split));
  methods.insert("join".to_string(), Rc::new(join));
}
