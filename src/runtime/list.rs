use std::{cell::RefCell, collections::HashMap, rc::Rc, vec};

use crate::{environment::Environment, object::Object};

use super::RuntimeFn;

fn unquote(args: &Vec<Object>) -> Object {
  match args.get(0) {
    Some(Object::Quote(o)) => {
      let o = o.clone();

      (*o).clone()
    },
    Some(o) => o.clone(),
    None => Object::Void,
  }
}

fn cdr(args: &Vec<Object>, _env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let list = unquote(args);

  match list {
    Object::List(list) => {
      let cdr = list.get(1..);

      match cdr {
        Some(cdr) => Ok(Object::List(cdr.to_vec())),
        None => Ok(Object::Void),
      }
    }
    _ => Ok(Object::Void),
  }
}

fn car(args: &Vec<Object>, _env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let list = unquote(args);

  match list {
    Object::List(list) => {
      let car = list.get(0);

      match car {
        Some(car) => Ok(car.clone()),
        None => Ok(Object::Void),
      }
    }
    _ => Ok(Object::Void),
  }
}

fn cons(args: &Vec<Object>, _env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let car = args.get(0);
  let cdr = args.get(1);

  let list = match cdr.clone().unwrap_or(&Object::Void) {
    Object::Quote(o) => {
      let o = o.clone();

      (*o).clone()
    },
    o => o.clone(),
  };

  match car {
    Some(car) => match list {
      Object::List(list) => {
        let mut list = list.clone();

        list.insert(0, car.clone());

        Ok(Object::List(list))
      }
      _ => Ok(Object::List(vec![car.clone()])),
    },
    None => Err("Expects at least one argument".to_string()),
  }
}

pub fn load_list_fns(methods: &mut HashMap<String, Rc<RuntimeFn>>) {
  methods.insert("cdr".to_string(), Rc::new(cdr));
  methods.insert("car".to_string(), Rc::new(car));
  methods.insert("cons".to_string(), Rc::new(cons));
}
