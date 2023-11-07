use crate::lexer::*;
use crate::object::Object;

use std::error::Error;
use std::fmt;
use std::rc::Rc;

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
      "define" | "defun" | "lambda" | "let" | "do" | "eval" => Object::Keyword(word),
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
  let mut list: Vec<Object> = Vec::new();

  while !tokens.is_empty() {
    let token = tokens.pop();
    if token == None {
      return Err(ParseError {
        err: format!("Insufficient tokens"),
      });
    }

    let token = token.unwrap();

    match token {
      Token::LParen => {
        let sub_list = parse_list(tokens)?;
        list.push(sub_list);
      }
      Token::RParen => {
        return Ok(Object::List(list));
      }
      Token::Quote => {
        let quoted_token = tokens.pop();
        if quoted_token == None {
          return Err(ParseError {
            err: format!("Insufficient tokens"),
          });
        }

        let mut quoted_token = quoted_token.unwrap();

        let mut quote_count = 1;

        while let Token::Quote = quoted_token {
          quote_count += 1;
          quoted_token = tokens.pop().unwrap();
        }

        let object = match quoted_token {
          Token::LParen => {
            let mut paren_count = 1;
            let mut quoted_tokens = vec![];

            while paren_count > 0 {
              let token = tokens.pop();
              if token == None {
                return Err(ParseError {
                  err: format!("Insufficient tokens"),
                });
              }

              let token = token.unwrap();
              quoted_tokens.insert(0, token.clone());

              match token {
                Token::LParen => paren_count += 1,
                Token::RParen => paren_count -= 1,
                _ => (),
              }
            }

            parse_list(&mut quoted_tokens)
          }
          Token::RParen => {
            return Err(ParseError {
              err: format!("Unexpected RParen after quote"),
            });
          }
          token => token_to_object(token.clone()),
        };

        let mut quoted_object: Object = object?;
        for _ in 0..quote_count {
          quoted_object = Object::Quote(Rc::new(quoted_object));
        }

        list.push(quoted_object);
      }
      token => {
        let object = token_to_object(token.clone())?;

        list.push(object);
      }
    }
  }

  match list.len() {
    0 => Ok(Object::List(Vec::new())),
    1 => Ok(list[0].clone()),
    _ => Ok(Object::List(list)),
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
