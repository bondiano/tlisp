use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Integer(i64),
  Float(f64),
  String(String),
  Operator(String),
  Keyword(String),
  Symbol(String),
  Quote,
  Cond,
  LParen,
  RParen,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use Token::*;
    f.write_str(
      (match self {
        Integer(n) => format!("{}", n),
        Float(n) => format!("{}", n),
        Operator(s) => format!("{}", s),
        String(s) => format!("{}", s),
        Symbol(s) => format!("{}", s),
        Quote => format!("'"),
        LParen => format!("("),
        RParen => format!(")"),
        Cond => format!("cond"),
        Keyword(s) => format!("{}", s),
      })
      .as_str(),
    )
  }
}

#[derive(Debug)]
pub struct TokenError {
  err: String,
}

impl Error for TokenError {}

impl fmt::Display for TokenError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Tokenization error: {}", self.err)
  }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
  let mut tokens = Vec::new();
  let mut chars = input.chars().collect::<Vec<char>>();

  if chars.is_empty() {
    return Ok(tokens);
  }

  while chars.len() > 0 {
    let mut ch = chars.remove(0);
    match ch {
      '(' => tokens.push(Token::LParen),
      ')' => tokens.push(Token::RParen),
      '"' => {
        let mut word = String::new();
        while chars.len() > 0 && chars[0] != '"' {
          word.push(chars.remove(0));
        }

        if chars.len() > 0 && chars[0] == '"' {
          chars.remove(0);
        } else {
          return Err(TokenError {
            err: format!("Unterminated string: {}", word),
          });
        }

        tokens.push(Token::String(word));
      }
      ';' => {
        while chars.len() > 0 && chars[0] != '\n' {
          chars.remove(0);
        }
      }
      '\'' => {
        tokens.push(Token::Quote);
      }
      ' ' | '\n' | '\t' => continue,
      _ => {
        let mut word = String::from(ch);

        while chars.len() > 0 && !ch.is_whitespace() && ch != '(' && ch != ')' {
          let peek = chars[0];
          if peek == '(' || peek == ')' {
            break;
          }

          ch = chars.remove(0);
          word.push(ch);
        }

        let word = String::from(word.trim());

        if !word.is_empty() {
          let token = if let Ok(i) = word.parse::<i64>() {
            Token::Integer(i)
          } else if let Ok(f) = word.parse::<f64>() {
            Token::Float(f)
          } else {
            match word.as_str() {
              "define" | "defun" | "lambda" | "let" | "do" | "eval" => Token::Keyword(word),
              "+" | "-" | "*" | "/" | "<" | ">" | "=" | "==" | "%" | "or" | "and" => {
                Token::Operator(word)
              }
              "cond" => Token::Cond,
              _ => Token::Symbol(word),
            }
          };

          tokens.push(token);
        }
      }
    }
  }

  Ok(tokens)
}

#[cfg(test)]
mod lexer_tests {
  use super::*;

  #[test]
  fn test_add() {
    let program = "(+ 2 2)";
    let tokens = tokenize(program).unwrap();
    assert_eq!(
      tokens,
      vec![
        Token::LParen,
        Token::Operator("+".to_string()),
        Token::Integer(2),
        Token::Integer(2),
        Token::RParen,
      ]
    );
  }

  #[test]
  fn test_quotation() {
    let program = "'(1  2 3)";
    let tokens = tokenize(program).unwrap();
    assert_eq!(
      tokens,
      vec![
        Token::Quote,
        Token::LParen,
        Token::Integer(1),
        Token::Integer(2),
        Token::Integer(3),
        Token::RParen,
      ]
    )
  }

  #[test]
  fn test_symbol() {
    let list = tokenize("#t").unwrap();
    assert_eq!(list, vec![Token::Symbol("#t".to_string())])
  }
}
