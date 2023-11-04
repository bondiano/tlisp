use crate::lexer::*;
use crate::object::Object;

use std::error::Error;
use std::fmt;

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
      Token::BinaryOp(b) => list.push(Object::BinaryOp(b)),
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
        Object::BinaryOp("+".to_string()),
        Object::Integer(1),
        Object::Integer(2),
      ])
    );
  }
}
