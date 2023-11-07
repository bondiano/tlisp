mod list;

use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use crate::{environment::Environment, eval::eval_object, object::Object};

pub type RuntimeFn = dyn Fn(&Vec<Object>, &mut Rc<RefCell<Environment>>) -> Result<Object, String>;

#[derive(Clone)]
pub struct Runtime {
  methods: Rc<HashMap<String, Rc<RuntimeFn>>>,
}

impl Debug for Runtime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let methods_str = self
      .methods
      .iter()
      .map(|(k, _)| k.clone())
      .collect::<Vec<String>>()
      .join(", ");

    write!(f, "Runtime {{ methods: {} }}", methods_str)
  }
}

fn debug(args: &Vec<Object>, _env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  println!("{:?}", args);

  Ok(Object::Void)
}

fn eval_eval(args: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let param = args.get(0);

  if param.is_none() {
    return Err(format!("Invalid number of arguments for eval"));
  }

  let param = param.unwrap();

  let unquoted = match param {
    Object::Quote(o) => &o,
    o => o,
  };

  let unquoted = match unquoted {
    Object::List(list) => {
      let mut unquoted_params = Vec::new();
      for obj in list {
        let result = match obj {
          Object::Quote(o) => &o,
          o => o,
        };

        unquoted_params.push(result.clone());
      }
      Object::List(unquoted_params)
    }
    o => o.clone(),
  };

  eval_object(&unquoted, env)
}

impl Runtime {
  pub fn new() -> Runtime {
    let mut methods: HashMap<String, Rc<RuntimeFn>> = HashMap::new();

    methods.insert("debug".to_string(), Rc::new(debug));
    methods.insert("eval".to_string(), Rc::new(eval_eval));

    list::load_list_fns(&mut methods);

    Runtime {
      methods: Rc::new(methods),
    }
  }

  pub fn get_method(&self, name: &str) -> Option<&Rc<RuntimeFn>> {
    self.methods.get(name)
  }
}
