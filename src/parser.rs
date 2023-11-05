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

fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {
  let token = tokens.pop();
  if token != Some(Token::LParen) {
    return Err(ParseError {
      err: format!("Expected LParen, found {:?}", token),
    });
  }

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
      Token::Keyword(k) => list.push(Object::Keyword(k)),
      Token::If => list.push(Object::If),
      Token::Operator(b) => list.push(Object::Operator(b)),
      Token::Integer(n) => list.push(Object::Integer(n)),
      Token::Float(f) => list.push(Object::Float(f)),
      Token::String(s) => list.push(Object::String(s)),
      Token::Symbol(s) => list.push(Object::Symbol(s)),
      Token::LParen => {
        tokens.push(Token::LParen);
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

        let mut quoted_tokens = vec![quoted_token.unwrap()];

        match quoted_tokens[0] {
          Token::LParen => {
            let mut paren_count = 1;
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
          }
          ref token => quoted_tokens.insert(0, token.clone()),
        }

        let sub_list = parse_list(&mut quoted_tokens)?;

        list.push(Object::Quote(Rc::new(sub_list)));
      },
    }
  }

  Ok(Object::List(list))
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
  fn test_quotation() {
    let list = parse("(do
      '(1 2 3))").unwrap();
    assert_eq!(
      list,
      Object::List(vec![
        Object::Keyword("do".to_string()),
        Object::Quote(Rc::new(Object::List(vec![
          Object::Integer(1),
          Object::Integer(2),
          Object::Integer(3),
        ])))
      ])
    )
  }
}
