use crate::lexer::*;
use crate::object::Object;

use std::{fmt, vec, error::Error, rc::Rc};

#[derive(Debug)]
pub struct ParseError {
  err: String,
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Parse error: {}", self.err)
  }
}

impl Error for ParseError {}

fn token_to_object(t: Token) -> Result<Object, ParseError> {
  let object = match t {
    Token::Integer(n) => Object::Integer(n),
    Token::Float(f) => Object::Float(f),
    Token::String(s) => Object::String(s),
    Token::Symbol(word) => match word.as_str() {
      "define" | "defun" | "lambda" | "let" | "do" => Object::Keyword(word),
      "+" | "-" | "*" | "/" | "<" | ">" | "=" | "==" | "%" | "or" | "and" => Object::Operator(word),
      "cond" => Object::Cond,
      _ => Object::Symbol(word),
    },
    _ => {
      return Err(ParseError {
        err: format!("Unexpected token: {:?}", t),
      })
    }
  };

  return Ok(object);
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {
  let mut stack: Vec<Object> = Vec::new();

  for token in tokens.iter() {
    match token {
      Token::RParen => {
        stack.push(Object::List(vec![]));
      }
      Token::LParen => {
        let sublist = match stack.pop() {
          Some(o) => o,
          None => {
            return Err(ParseError {
              err: format!("Unexpected token: {:?}", token),
            })
          }
        };

        let to = match stack.last_mut() {
          Some(Object::List(l)) => l,
          _ => &mut stack,
        };

        match sublist {
          Object::List(l) => {
            let mut l = l.clone();
            l.reverse();

            to.push(Object::List(l.to_vec()));
          }
          o => {
            to.push(o);
          }
        }
      }
      Token::Quote => {
      }
      token => {
        let object = token_to_object(token.clone())?;

        let to = match stack.len() {
          0 => &mut stack,
          _ => {
            let last = stack.last_mut().unwrap();
            match last {
              Object::List(l) => l,
              _ => &mut stack,
            }
          }
        };

        to.push(object);
      }
    }
  }

  match stack.len() {
    0 => Ok(Object::Void),
    1 => Ok(stack.pop().unwrap()),
    _ => Ok(Object::List(stack)),
  }
}

pub fn parse(program: &str) -> Result<Object, ParseError> {
  let token_result = tokenize(program);

  let mut tokens = token_result
    .unwrap()
    .into_iter()
    .rev()
    .collect::<Vec<Token>>();
  let parsed_list = parse_list(&mut tokens)?;
  Ok(parsed_list)
}

#[cfg(test)]
mod lexer_tests {
  use super::*;

  #[test]
  fn test_add() {
    let list = parse("(+ 1 2)").unwrap();
    assert_eq!(
      list,
      Object::List(vec![
        Object::Operator("+".to_string()),
        Object::Integer(1),
        Object::Integer(2),
      ])
    );
  }

  #[test]
  fn test_nested_add() {
    let list = parse("(+ 1 (+ 2 3))").unwrap();
    assert_eq!(
      list,
      Object::List(vec![
        Object::Operator("+".to_string()),
        Object::Integer(1),
        Object::List(vec![
          Object::Operator("+".to_string()),
          Object::Integer(2),
          Object::Integer(3),
        ]),
      ])
    );
  }

  #[test]
  fn test_nested_operations() {
    let list = parse("(add 1 (/ 3 (* 10 2)) (+ 2 3))").unwrap();

    assert_eq!(
      list,
      Object::List(vec![
        Object::Symbol("add".to_string()),
        Object::Integer(1),
        Object::List(vec![
          Object::Operator("/".to_string()),
          Object::Integer(3),
          Object::List(vec![
            Object::Operator("*".to_string()),
            Object::Integer(10),
            Object::Integer(2),
          ]),
        ]),
        Object::List(vec![
          Object::Operator("+".to_string()),
          Object::Integer(2),
          Object::Integer(3),
        ]),
      ])
    );
  }

  #[test]
  fn test_symbol() {
    let list = parse("#t").unwrap();
    assert_eq!(list, Object::Symbol("#t".to_string()))
  }

  #[test]
  fn test_quotation() {
    let list = parse("'(1 2 3)").unwrap();
    assert_eq!(
      list,
      Object::Quote(Rc::new(Object::List(vec![
        Object::Integer(1),
        Object::Integer(2),
        Object::Integer(3),
      ])))
    )
  }

  #[test]
  fn test_quotation_inside_list() {
    let list = parse("('a b)").unwrap();
    assert_eq!(
      list,
      Object::List(vec![
        Object::Quote(Rc::new(Object::Symbol("a".to_string()))),
        Object::Symbol("b".to_string())
      ])
    )
  }

  #[test]
  fn test_quotation_in_quoted_list() {
    let list = parse("'('('a b))").unwrap();

    assert_eq!(
      list,
      Object::Quote(Rc::new(Object::List(vec![Object::Quote(Rc::new(
        Object::List(vec![
          Object::Quote(Rc::new(Object::Symbol("a".to_string()))),
          Object::Symbol("b".to_string())
        ])
      ))])))
    )
  }

  #[test]
  fn test_multi_quotation() {
    let list = parse("''a").unwrap();

    assert_eq!(
      list,
      Object::Quote(Rc::new(Object::Quote(Rc::new(Object::Symbol(
        "a".to_string()
      )))))
    )
  }
}
