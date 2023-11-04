#[derive(Debug, PartialEq)]
pub enum Token {
  Integer(i64),
  Symbol(String),
  LParen,
  RParen,
}

pub fn tokenize(input: &str) -> Vec<Token> {
  let mut tokens: Vec<Token> = Vec::new();

  let program2 = input.replace("(", " ( ").replace(")", " ) ");
  let words: Vec<&str> = program2.split_whitespace().collect();

  for word in words {
    match word {
      "(" => tokens.push(Token::LParen),
      ")" => tokens.push(Token::RParen),
      _ => {
        let i = word.parse::<i64>();
        if i.is_ok() {
          tokens.push(Token::Integer(i.unwrap()));
        } else {
          tokens.push(Token::Symbol(word.to_string()));
        }
      }
    }
  }

  tokens
}

#[cfg(test)]
mod lexer_tests {
  use super::*;

  #[test]
  fn test_add() {
    let tokens = tokenize("(+ 2 2)");
    assert_eq!(
      tokens,
      vec![
        Token::LParen,
        Token::Symbol("+".to_string()),
        Token::Integer(2),
        Token::Integer(2),
        Token::RParen,
      ]
    );
  }
}
