use std::{cell::RefCell, rc::Rc};

use crate::environment::Environment;
use crate::object::Object;
use crate::operators;
use crate::parser::parse;

fn eval_symbol(s: &str, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let val = match s {
    "#t" => return Ok(Object::Bool(true)),
    "#f" => return Ok(Object::Bool(false)),
    "#nil" => return Ok(Object::Void),
    _ => env.borrow().get(s),
  };

  let val = match val {
    Some(val) => val,
    None => match env.borrow().get_runtime_fn(s) {
      Some(_f) => Object::Native(s.to_string()),
      None => return Err(format!("Unbound symbol: {}", s)),
    },
  };

  Ok(val.clone())
}

fn eval_do(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let mut result = Object::Void;
  let mut new_env = Rc::new(RefCell::new(Environment::extend(env.clone())));

  for obj in list[1..].iter() {
    result = eval_object(obj, &mut new_env)?;
  }
  Ok(result)
}

fn eval_define(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  if list.len() != 3 {
    return Err(format!("Invalid number of forms for define"));
  }

  let symbol = match &list[1] {
    Object::Symbol(s) => s,
    _ => return Err(format!("Invalid symbol for define")),
  };

  let value = eval_object(&list[2], env)?;
  env.borrow_mut().set(symbol, value);
  Ok(Object::Void)
}

fn eval_defun(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  if list.len() != 4 {
    return Err(format!("Invalid number of forms for defun"));
  }

  let name = match &list[1] {
    Object::Symbol(s) => s,
    _ => return Err(format!("Invalid symbol for defun")),
  };

  let params = match &list[2] {
    Object::List(l) => {
      let mut params = Vec::new();

      for obj in l {
        match obj {
          Object::Symbol(s) => params.push(s.clone()),
          _ => return Err(format!("Invalid lambda parameter {:?}", params)),
        }
      }

      params
    }
    _ => return Err(format!("Expected list of parameters")),
  };

  let body = list.get(3).unwrap().to_owned();

  let lambda = Object::Lambda(params, Box::new(body), env.clone());
  env.borrow_mut().set(&name, lambda);

  Ok(Object::Void)
}

fn eval_cond(
  list: &Vec<Object>,
  env: &mut Rc<RefCell<Environment>>,
) -> Result<Box<Object>, String> {
  if (list.len() % 2) != 1 {
    return Err(format!("Cond requires an even number of forms"));
  }

  let args_pairs = list[1..].chunks(2);
  for args_pair in args_pairs {
    let cond_obj = args_pair.get(0).unwrap_or(&Object::Void);
    let body_ob = args_pair.get(1).unwrap_or(&Object::Void);

    let cond_result = match eval_object(cond_obj, env)? {
      Object::Bool(b) => b,
      Object::Void => false,
      _ => true,
    };

    if cond_result {
      return Ok(Box::new(body_ob.clone()));
    }
  }

  Ok(Box::new(Object::Void))
}

fn eval_let(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let mut result = Object::Void;
  let mut bindings_env = Rc::new(RefCell::new(Environment::extend(env.clone())));

  if list.len() < 3 {
    return Err(format!("Invalid number of forms for let"));
  }

  let bindings = match list[1].clone() {
    Object::List(list) => list,
    _ => return Err(format!("Invalid bindings for let")),
  };

  for binding in bindings {
    let binding = match binding {
      Object::List(list) => list,
      _ => return Err(format!("Invalid binding for let")),
    };

    if binding.len() != 2 {
      return Err(format!("Invalid binding for let"));
    }

    let symbol = match &binding[0] {
      Object::Symbol(s) => s,
      _ => return Err(format!("Invalid symbol for let")),
    };

    let value = eval_object(&binding[1], &mut bindings_env)?;
    bindings_env.borrow_mut().set(symbol, value);
  }

  let mut new_env = Rc::new(RefCell::new(Environment::extend(env.clone())));
  new_env.borrow_mut().update(bindings_env);

  for obj in list[2..].iter() {
    result = eval_object(obj, &mut new_env)?;
  }
  Ok(result)
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
          _ => return Err(format!("Invalid lambda parameter {:?}", params)),
        }
      }

      params
    }
    _ => return Err(format!("Expected list of parameters")),
  };

  let body = list.get(2).unwrap().to_owned();

  Ok(Object::Lambda(params, Box::new(body), env.clone()))
}

fn eval_operator(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  if list.len() < 2 {
    let operator = list.get(0).unwrap_or(&Object::Void);
    return Err(format!(
      "Invalid number of arguments for operator {}",
      operator
    ));
  }
  let operator = list[0].clone();

  let mut params = list[1..].into_iter().map(|o| eval_object(&o, env));

  match operator {
    Object::Operator(s) => match s.as_str() {
      "+" => operators::sum(&mut params),
      "-" => operators::sub(&mut params),
      "*" => operators::mult(&mut params),
      "/" => operators::div(&mut params),
      "%" => operators::mod_(&mut params),
      "<" => operators::lt(&mut params),
      ">" => operators::gt(&mut params),
      "=" => operators::eq(&mut params),
      "==" => operators::strict_eq(&mut params),
      "and" => operators::and(&mut params),
      "or" => operators::or(&mut params),
      _ => Err(format!("Invalid infix operator: {}", s)),
    },
    _ => Err(format!("Operator must be a symbol")),
  }
}

fn eval_function_call(
  s: &str,
  list: &Vec<Object>,
  env: &mut Rc<RefCell<Environment>>,
) -> Result<(Box<Object>, Rc<RefCell<Environment>>), String> {
  let symbol = env.borrow().get(s);

  if symbol.is_none() {
    return Err(format!("Unbound symbol: {}", s));
  }

  let lambda = symbol.unwrap();
  match lambda {
    Object::Lambda(params, body, func_env) => {
      let new_env = Rc::new(RefCell::new(Environment::extend(func_env.clone())));

      for (i, param) in params.iter().enumerate() {
        let arg = list.get(i + 1);

        match arg {
          Some(arg) => {
            let val = eval_object(&arg, env)?;
            new_env.borrow_mut().set(param, val);
          }
          None => return Err(format!("Invalid number of arguments for {}", s)),
        }
      }

      Ok((body, new_env.clone()))
    }
    _ => {
      return Err(format!("{} is not a function", s));
    }
  }
}

fn eval_anonymus_function_call(
  list: &Vec<Object>,
  env: &mut Rc<RefCell<Environment>>,
) -> Result<(Box<Object>, Rc<RefCell<Environment>>), String> {
  let lambda = &list[0];
  match lambda {
    Object::Lambda(params, body, func_env) => {
      let new_env = Rc::new(RefCell::new(Environment::extend(func_env.clone())));

      for (i, param) in params.iter().enumerate() {
        let object = match list.get(i + 1) {
          Some(o) => o,
          None => return Err(format!("Invalid number of arguments for lambda")),
        };
        let val = eval_object(object, env)?;
        new_env.borrow_mut().set(param, val);
      }
      Ok((body.to_owned(), new_env))
    }
    _ => return Err(format!("Not a lambda")),
  }
}

fn eval_keyword(list: &Vec<Object>, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let head = &list[0];
  match head {
    Object::Keyword(s) => match s.as_str() {
      "define" => eval_define(&list, env),
      "defun" => eval_defun(&list, env),
      "lambda" => eval_function_definition(&list, env),
      "let" => eval_let(&list, env),
      "do" => eval_do(&list, env),
      _ => Err(format!("Unknown keyword: {}", s)),
    },
    _ => {
      return Err(format!("Invalid keyword: {}", head));
    }
  }
}

fn eval_native(
  s: &str,
  list: &Vec<Object>,
  env: &mut Rc<RefCell<Environment>>,
) -> Result<Object, String> {
  let f = env.borrow().get_runtime_fn(&s).unwrap();
  let mut params = Vec::new();

  let rest_params = list.get(1..).unwrap_or_default();

  for obj in rest_params {
    let result = eval_object(&obj, env)?;
    params.push(result);
  }

  return f(&params, env);
}

pub fn eval_object(obj: &Object, env: &mut Rc<RefCell<Environment>>) -> Result<Object, String> {
  let mut current_obj: Box<Object> = Box::new(obj.clone());
  let mut current_env = env.clone();

  loop {
    match *current_obj {
      Object::List(list) => {
        let head = &list[0];
        match head {
          Object::Operator(_op) => return eval_operator(&list, &mut current_env),
          Object::Keyword(_k) => return eval_keyword(&list, &mut current_env),
          Object::Cond => {
            current_obj = eval_cond(&list, &mut current_env)?;
            continue;
          }
          Object::Symbol(s) => {
            let symbol = eval_symbol(s, &mut current_env)?;

            match symbol {
              Object::Lambda(_, _, _) => {
                (current_obj, current_env) = eval_function_call(&s, &list, &mut current_env)?;
                continue;
              }
              Object::Native(_) => {
                return eval_native(&s, &list, &mut current_env);
              }
              _ => {
                current_obj = Box::new(symbol);
                continue;
              }
            }
          }
          Object::Lambda(_, _, _) => {
            (current_obj, current_env) = eval_anonymus_function_call(&list, &mut current_env)?;
            continue;
          }
          _ => {
            let mut new_list = Vec::new();
            for obj in list {
              let result = eval_object(&obj, &mut current_env)?;
              match result {
                Object::Void => {}
                _ => new_list.push(result),
              }
            }

            let head = new_list.first().unwrap_or(&Object::Void);
            return match head {
              Object::Void => Ok(Object::Void),
              Object::Bool(_) | Object::Integer(_) | Object::Float(_) | Object::String(_) => {
                Err(format!("Invalid head of list to call: {}", head))
              }
              _ => eval_object(&Object::List(new_list), &mut current_env),
            };
          }
        }
      }
      Object::Bool(_) => return Ok(obj.clone()),
      Object::Integer(n) => return Ok(Object::Integer(n)),
      Object::Float(n) => return Ok(Object::Float(n)),
      Object::String(s) => return Ok(Object::String(s.to_string())),
      Object::Symbol(s) => return eval_symbol(&s, &mut current_env),
      Object::Lambda(_params, _body, _func_env) => return Ok(Object::Void),
      Object::Quote(o) => return Ok(Object::Quote(o)),
      Object::Operator(o) => return Ok(Object::Operator(o)),
      Object::Keyword(k) => return Ok(Object::Keyword(k)),
      Object::Void => return Ok(Object::Void),
      Object::Cond => return Ok(Object::Cond),
      Object::Native(s) => return Ok(Object::Native(s)),
    }
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
  use crate::runtime::Runtime;

  use super::*;

  #[test]
  fn test_simple_add() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let result = eval("(+ 1 2)", &mut env).unwrap();
    assert_eq!(result, Object::Integer(3));
  }

  #[test]
  fn test_lambda_add() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let result = eval("((lambda (x y) (+ x y)) 2 5)", &mut env).unwrap();
    assert_eq!(result, Object::Integer(7));
  }

  #[test]
  fn test_area_of_a_circle() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(do
      (define r 10)
      (define pi 314)
      (* pi (* r r)))";
    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer((314 * 10 * 10) as i64));
  }

  #[test]
  fn test_sqr_function() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(do
      (define sqr (lambda (r) (* r r)))
      (sqr 10))";
    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer((10 * 10) as i64));
  }

  #[test]
  fn test_fibonacci() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(do
      (define fib
        (lambda (n)
          (cond (< n 2) 1
                #t (+ (fib (- n 1)) (fib (- n 2))))))
      (fib 10))";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer((89) as i64));
  }

  #[test]
  fn test_factorial() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(do
      (define fact
        (lambda (n)
          (cond (< n 1) 1
                #t (* n (fact (- n 1))))))
      (fact 5))";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer((120) as i64));
  }

  #[test]
  fn test_circle_area_function() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(do
      (define pi 3.14)
      (define r 10)
      (define sqr (lambda (r) (* r r)))
      (define area (lambda (r) (* pi (sqr r))))
      (area r))";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Float(3.14 * 10.0 * 10.0));
  }

  #[test]
  fn test_closure() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(do
      (define add-n
          (lambda (n)
            (lambda (a) (+ n a))))
      (define add-5 (add-n 5))
      (add-5 10))";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer(15));
  }

  #[test]
  fn test_return_function() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));

    let program = "(defun const (n) (lambda (a) n))";
    eval(program, &mut env).unwrap();

    let program = "(const 10)";
    let result = eval(program, &mut env).unwrap();

    let mut expected_env: Environment = Environment::extend(env.clone());

    expected_env.set("n", Object::Integer(10));

    assert_eq!(
      result,
      Object::Lambda(
        vec!["a".to_string()],
        Box::new(Object::Symbol("n".to_string())),
        Rc::new(RefCell::new(expected_env))
      )
    );
  }

  #[test]
  fn test_cond() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(do
      (define x 40)
      (cond (= x 10) 1
            (= x 20) 2
            #t 3))";
    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer(3));
  }

  #[test]
  fn test_tail_recursion() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(do
      (define sum-n
          (lambda (n a)
            (cond (= n 0) a
                  #t (sum-n (- n 1) (+ n a)))))
      (sum-n 5000 2))";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer((12502502) as i64));
  }

  #[test]
  fn test_let() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "
        (let ((x 2) (y 3))
            (let ((x 7)
                  (z (+ x y)))
                (* z x)))";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer(70));
  }

  #[test]
  fn test_let_tail() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(do
      (define fact
        (lambda (n)
          (let ((fact-iter
                (lambda (n a)
                  (cond
                      (= n 0) a
                      #t (fact-iter (- n 1) (* n a))))))
            (fact-iter n 1))))
          (fact 5))";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer(120));
  }

  #[test]
  fn test_circle_area_no_lambda() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(do
      (define pi 314)
      (define r 10)
      (defun sqr (r) (* r r))
      (defun area (r) (* pi (sqr r)))
      (area r))";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer((314 * 10 * 10) as i64));
  }

  #[test]
  fn test_eval_to_eval_quoted() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(eval '(+ 1 2 3))";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer(6));
  }

  #[test]
  fn test_eval_quoted_function() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(do
      (defun sqr (r) (* r r))
      (eval '(sqr 10)))";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer(100));
  }

  #[test]
  fn test_multi_arguments_operators() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(= 1 1 1 1 1 1 1 1 1 1 1 1 1 1)";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Bool(true));

    let program = "(= 1 1 1 1 1 1 1 1 1 1 1 1 1 2)";
    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Bool(false));

    let program = "(+ 10 11 12 13 14 15 16 17 18 19 20)";
    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer(165));
  }

  #[test]
  fn test_evaluate_operator_expression() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "((cond #f = #t *) 3 4)";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer(12));
  }

  #[test]
  fn test_or_operator() {
    let runtime = Runtime::new();
    let mut env = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(or 1 2 3)";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer(1));

    let program = "(or #f 2 3)";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer(2));

    let program = "(or #f #nil 3)";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer(3));
  }

  #[test]
  fn test_and_operator() {
    let runtime = Runtime::new();
    let mut env: Rc<RefCell<Environment>> = Rc::new(RefCell::new(Environment::new(runtime)));
    let program = "(and 1 2 3)";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Integer(3));

    let program = "(and 1 2 #f)";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Bool(false));

    let program = "(and 1 #nil #f)";

    let result = eval(program, &mut env).unwrap();
    assert_eq!(result, Object::Void);
  }
}
