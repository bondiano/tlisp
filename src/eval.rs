use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use crate::environment::Environment;
use crate::object::Object;
use crate::parser::parse;

fn eval_symbol(s: &str, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let val = env.borrow_mut().get(s);
  if val.is_none() {
    return Err(format!("Unbound symbol: {}", s));
  }
  Ok(val.unwrap().clone())
}

fn eval_define(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  if list.len() != 3 {
    return Err(format!("Invalid number of arguments for define"));
  }

  let symbol = match &list[1] {
    Object::Symbol(s) => s,
    _ => return Err(format!("Invalid symbol for define")),
  };

  let value = eval_object(&list[2], env)?;
  env.borrow_mut().set(symbol, value);
  Ok(Object::Void)
}

fn eval_if(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let condition = eval_object(&list[1], env)?;
  match condition {
    Object::Bool(true) => eval_object(&list[2], env),
    Object::Bool(false) => eval_object(&list[3], env),
    _ => Err(format!("Expected boolean")),
  }
}

fn eval_function_definition(
  list: &Vec<Object>,
  env: &mut Rc<RefCell<Environment>>,
) -> Result<Object, String> {
  let params = match &list[1] {
    Object::List(list) => {
      let mut params = Vec::new();

      for obj in list {
        match obj {
          Object::Symbol(s) => params.push(s.clone()),
          _ => return Err(format!("Expected list of symbols")),
        }
      }

      params
    }
    _ => return Err(format!("Expected list of parameters")),
  };

  let body = match &list[2] {
    Object::List(list) => list.clone(),
    _ => return Err(format!("Expected list of body")),
  };

  Ok(Object::Lambda(params, body, env.clone()))
}

fn eval_binary_op(
  list: &Vec<Object>,
  env: &mut Rc<RefCell<Environment>>,
) -> Result<Object, String> {
  if list.len() != 3 {
    return Err(format!("Invalid number of arguments for infix operator"));
  }
  let operator = list[0].clone();
  let left = &eval_object(&list[1].clone(), env)?;
  let right = &eval_object(&list[2].clone(), env)?;

  match operator {
    Object::BinaryOp(s) => match s.as_str() {
      "+" => match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l + r)),
        (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l + r)),
        (Object::Integer(l), Object::Float(r)) => Ok(Object::Float(*l as f64 + r)),
        (Object::Float(l), Object::Integer(r)) => Ok(Object::Float(l + *r as f64)),
        (Object::String(l), Object::String(r)) => Ok(Object::String(l.to_owned() + &*r)),
        _ => Err(format!("Invalid types for + operator {} {}", left, right)),
      },
      "-" => match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l - r)),
        (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l - r)),
        (Object::Integer(l), Object::Float(r)) => Ok(Object::Float(*l as f64 - r)),
        (Object::Float(l), Object::Integer(r)) => Ok(Object::Float(l - *r as f64)),
        _ => Err(format!("Invalid types for - operator {} {}", left, right)),
      },
      "*" => match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l * r)),
        (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l * r)),
        (Object::Integer(l), Object::Float(r)) => Ok(Object::Float(*l as f64 * r)),
        (Object::Float(l), Object::Integer(r)) => Ok(Object::Float(l * (*r) as f64)),
        _ => Err(format!("Invalid types for * operator {} {}", left, right)),
      },
      "/" => match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l / r)),
        (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l / r)),
        (Object::Integer(l), Object::Float(r)) => Ok(Object::Float(*l as f64 / r)),
        (Object::Float(l), Object::Integer(r)) => Ok(Object::Float(l / (*r) as f64)),
        _ => Err(format!("Invalid types for / operator {} {}", left, right)),
      },
      "%" => match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l % r)),
        (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l % r)),
        (Object::Integer(l), Object::Float(r)) => Ok(Object::Float(*l as f64 % r)),
        (Object::Float(l), Object::Integer(r)) => Ok(Object::Float(l % (*r) as f64)),
        _ => Err(format!("Invalid types for % operator {} {}", left, right)),
      },
      "<" => match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Bool(l < r)),
        (Object::Float(l), Object::Float(r)) => Ok(Object::Bool(l < r)),
        (Object::Integer(l), Object::Float(r)) => Ok(Object::Bool((*l as f64) < *r)),
        (Object::Float(l), Object::Integer(r)) => Ok(Object::Bool(l < &(*r as f64))),
        (Object::String(l), Object::String(r)) => Ok(Object::Bool(l.cmp(&r) == Ordering::Less)),
        _ => Err(format!("Invalid types for < operator {} {}", left, right)),
      },
      ">" => match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Bool(l > r)),
        (Object::Float(l), Object::Float(r)) => Ok(Object::Bool(l > r)),
        (Object::Integer(l), Object::Float(r)) => Ok(Object::Bool(*l as f64 > *r)),
        (Object::Float(l), Object::Integer(r)) => Ok(Object::Bool(l > &(*r as f64))),
        (Object::String(l), Object::String(r)) => Ok(Object::Bool(l.cmp(&r) == Ordering::Greater)),
        _ => Err(format!("Invalid types for > operator {} {}", left, right)),
      },
      "=" => match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Bool(l == r)),
        (Object::String(l), Object::String(r)) => Ok(Object::Bool(l == r)),
        _ => Err(format!("Invalid types for = operator {} {}", left, right)),
      },
      "!=" => match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Bool(l != r)),
        (Object::Float(l), Object::Float(r)) => Ok(Object::Bool(l != r)),
        (Object::Integer(l), Object::Float(r)) => Ok(Object::Bool(*l as f64 != *r)),
        (Object::Float(l), Object::Integer(r)) => Ok(Object::Bool(*l != (*r) as f64)),
        (Object::String(l), Object::String(r)) => Ok(Object::Bool(l.cmp(&r) != Ordering::Equal)),
        _ => Err(format!("Invalid types for != operator {} {}", left, right)),
      },
      "and" => match (left, right) {
        (Object::Bool(l), Object::Bool(r)) => Ok(Object::Bool(*l && *r)),
        _ => Err(format!("Invalid types for & operator {} {}", left, right)),
      },
      "or" => match (left, right) {
        (Object::Bool(l), Object::Bool(r)) => Ok(Object::Bool(*l || *r)),
        _ => Err(format!("Invalid types for | operator {} {}", left, right)),
      },
      _ => Err(format!("Invalid infix operator: {}", s)),
    },
    _ => Err(format!("Operator must be a symbol")),
  }
}

fn eval_function_call(
  s: &str,
  list: &Vec<Object>,
  env: &mut Rc<RefCell<Environment>>,
) -> Result<Object, String> {
  let lambda = env.borrow_mut().get(s);
  if lambda.is_none() {
    return Err(format!("Unbound symbol: {}", s));
  }

  let lambda = lambda.unwrap();
  match lambda {
    Object::Lambda(params, body, func_env) => {
      let mut new_env = Rc::new(RefCell::new(Environment::extend(func_env.clone())));

      for (i, param) in params.iter().enumerate() {
        let val = eval_object(&list[i + 1], env)?;
        new_env.borrow_mut().set(param, val);
      }
      return eval_object(&Object::List(body), &mut new_env);
    }
    _ => Err(format!("{} is not a lambda", s)),
  }
}

fn eval_anonymus_function_call(
  list: &Vec<Object>,
  env: &mut Rc<RefCell<Environment>>,
) -> Result<Object, String> {
  let lambda = &list[0];
  match lambda {
    Object::Lambda(params, body, func_env) => {
      let mut new_env = Rc::new(RefCell::new(Environment::extend(func_env.clone())));
      for (i, param) in params.iter().enumerate() {
        let val = eval_object(&list[i + 1], env)?;
        new_env.borrow_mut().set(param, val);
      }

      eval_object(&Object::List(body.to_vec()), &mut new_env)
    }
    _ => return Err(format!("Not a lambda")),
  }
}

fn eval_list(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let head = &list[0];
  match head {
    Object::Keyword(_) => eval_keyword(list, env),
    Object::BinaryOp(_) => eval_binary_op(list, env),
    Object::If => eval_if(list, env),
    Object::Lambda(_, _, _) => eval_function_definition(&list, env),
    Object::Symbol(s) => eval_function_call(&s, &list, env),
    _ => {
      let mut new_list = Vec::new();
      for obj in list {
        let result = eval_object(obj, env)?;
        match result {
          Object::Void => {}
          _ => new_list.push(result),
        }
      }

      let head = new_list.get(0).unwrap_or(&Object::Void);
      match head {
        Object::Lambda(_, _, _) => {
          return eval_anonymus_function_call(&new_list, env);
        }
        _ => Ok(Object::List(new_list)),
      }
    }
  }
}

fn eval_keyword(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let head = &list[0];
  match head {
    Object::Keyword(s) => match s.as_str() {
      "define" => eval_define(&list, env),
      "lambda" => eval_function_definition(&list, env),
      _ => Err(format!("Unknown keyword: {}", s)),
    },
    _ => {
      return Err(format!("Invalid keyword: {}", head));
    }
  }
}

fn eval_object(obj: &Object, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  match obj {
    Object::Void => Ok(Object::Void),
    Object::Lambda(_params, _body, _func_env) => Ok(Object::Void),
    Object::Bool(_) => Ok(obj.clone()),
    Object::Integer(n) => Ok(Object::Integer(*n)),
    Object::Float(f) => Ok(Object::Float(*f)),
    Object::String(s) => Ok(Object::String(s.clone())),
    Object::Symbol(s) => eval_symbol(s, env),
    Object::List(list) => eval_list(list, env),
    _ => return Err(format!("Invalid object: {:?}", obj)),
  }
}

pub fn eval(program: &str, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let parsed_list = parse(program);
  if parsed_list.is_err() {
    return Err(format!("{}", parsed_list.err().unwrap()));
  }
  eval_object(&parsed_list.unwrap(), env)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_add() {
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let result = eval("(+ 1 2)", &mut env).unwrap();
    assert_eq!(result, Object::Integer(3));
  }

  #[test]
  fn test_lambda_add() {
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let result = eval("((lambda (x y) (+ x y)) 2 5)", &mut env).unwrap();
    assert_eq!(result, Object::Integer(7));
  }

  #[test]
  fn test_area_of_a_circle() {
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let program = "(
                        (define r 10)
                        (define pi 314)
                        (* pi (* r r))
                      )";
    let result = eval(program, &mut env).unwrap();
    assert_eq!(
      result,
      Object::List(vec![Object::Integer((314 * 10 * 10) as i64)])
    );
  }

  #[test]
  fn test_sqr_function() {
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let program = "(
                        (define sqr (lambda (r) (* r r)))
                        (sqr 10)
                       )";
    let result = eval(program, &mut env).unwrap();
    assert_eq!(
      result,
      Object::List(vec![Object::Integer((10 * 10) as i64)])
    );
  }

  #[test]
  fn test_fibonaci() {
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let program = "
            (
                (define fib (lambda (n) (if (< n 2) 1 (+ (fib (- n 1)) (fib (- n 2))))))
                (fib 10)
            )
        ";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::List(vec![Object::Integer((89) as i64)]));
  }

  #[test]
  fn test_factorial() {
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let program = "
            (
                (define fact (lambda (n) (if (< n 1) 1 (* n (fact (- n 1))))))
                (fact 5)
            )
        ";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::List(vec![Object::Integer((120) as i64)]));
  }

  #[test]
  fn test_circle_area_function() {
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let program = "
            (
                (define pi 3.14)
                (define r 10)
                (define sqr (lambda (r) (* r r)))
                (define area (lambda (r) (* pi (sqr r))))
                (area r)
            )
        ";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(
      result,
      Object::List(vec![Object::Float(3.14 * 10.0 * 10.0)])
    );
  }

  #[test]
  fn test_closure() {
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let program = "
            (
                (define add-n
                   (lambda (n)
                      (lambda (a) (+ n a))))
                (define add-5 (add-n 5))
                (add-5 10)
            )
        ";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(
      result,
      Object::List(vec![Object::Integer(15)])
    );
  }
}
