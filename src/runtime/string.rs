use std::{cell::RefCell, collections::HashMap, rc::Rc};
use dyn_fmt;
use crate::{environment::Environment, object::Object};

use super::RuntimeFn;

fn format_(args: &Vec<Object>, _env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let arg = args.get(0).unwrap();

  let rest = args.get(1..).unwrap();

  match arg {
    Object::String(str) => {
      Ok(Object::String(dyn_fmt::AsStrFormatExt::format(str, rest).to_string()))
    },
    _ => Ok(Object::Void)
  }
}

pub fn load_string_fns(methods: &mut HashMap<String, Rc<RuntimeFn>>) {
  methods.insert("format".to_string(), Rc::new(format_));
}
