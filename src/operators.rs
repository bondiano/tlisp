use crate::object::Object;

pub fn sum<I: Iterator<Item = Result<Object, String>>>(params: &mut I) -> Result<Object, String> {
  let mut sum = params.next().unwrap()?;

  for param in params {
    sum = match sum {
      Object::Integer(n) => match param? {
        Object::Integer(m) => Object::Integer(m + n),
        Object::Float(m) => Object::Float(m + n as f64),
        param => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::Float(n) => match param? {
        Object::Integer(m) => Object::Float(m as f64 + n),
        Object::Float(m) => Object::Float(m + n),
        param => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::String(s) => match param? {
        Object::String(t) => Object::String(format!("{}{}", s, t)),
        param => return Err(format!("Expected string, found {}", param)),
      },
      _ => return Err(format!("{} could not be added", sum)),
    }
  }

  Ok(sum)
}

pub fn sub<I: Iterator<Item = Result<Object, String>>>(params: &mut I) -> Result<Object, String> {
  let mut diff: Object = params.next().unwrap()?;

  let next = params.next();

  if next.is_none() {
    diff = match diff {
      Object::Integer(n) => Object::Integer(-n),
      Object::Float(n) => Object::Float(-n),
      _ => return Err(format!("Expected int or float, found {}", diff)),
    };

    return Ok(diff);
  }

  let rest_params = std::iter::once(next.unwrap()).chain(params);

  for param in rest_params {
    diff = match diff {
      Object::Integer(n) => match param? {
        Object::Integer(m) => Object::Integer(n - m),
        Object::Float(m) => Object::Float(n as f64 - m),
        param => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::Float(n) => match param? {
        Object::Integer(m) => Object::Float(n - m as f64),
        Object::Float(m) => Object::Float(n - m),
        param => return Err(format!("Expected int or float, found {}", param)),
      },
      _ => return Err(format!("{} could not be subtracted", diff)),
    }
  }

  Ok(diff)
}

pub fn mult<I: Iterator<Item = Result<Object, String>>>(params: &mut I) -> Result<Object, String> {
  let mut product: Object = params.next().unwrap()?;

  for param in params {
    product = match product {
      Object::Integer(n) => match param? {
        Object::Integer(m) => Object::Integer(m * n),
        Object::Float(m) => Object::Float(m * n as f64),
        param => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::Float(n) => match param? {
        Object::Integer(m) => Object::Float(m as f64 * n),
        Object::Float(m) => Object::Float(m * n),
        param => return Err(format!("Expected int or float, found {}", param)),
      },
      _ => return Err(format!("{} could not be multiplied", product)),
    }
  }

  Ok(product)
}

pub fn div<I: Iterator<Item = Result<Object, String>>>(params: &mut I) -> Result<Object, String> {
  let mut quotient: Object = params.next().unwrap()?;

  for param in params {
    quotient = match quotient {
      Object::Integer(n) => match param? {
        Object::Integer(m) => Object::Integer(n / m),
        Object::Float(m) => Object::Float(n as f64 / m),
        param => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::Float(n) => match param? {
        Object::Integer(m) => Object::Float(n / m as f64),
        Object::Float(m) => Object::Float(n / m),
        param => return Err(format!("Expected int or float, found {}", param)),
      },
      _ => return Err(format!("{} could not be divided", quotient)),
    }
  }

  Ok(quotient)
}

pub fn mod_<I: Iterator<Item = Result<Object, String>>>(params: &mut I) -> Result<Object, String> {
  let mut remainder: Object = params.next().unwrap()?;

  for param in params {
    remainder = match remainder {
      Object::Integer(n) => match param? {
        Object::Integer(m) => Object::Integer(n % m),
        Object::Float(m) => Object::Float(n as f64 % m),
        param => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::Float(n) => match param? {
        Object::Integer(m) => Object::Float(n % m as f64),
        Object::Float(m) => Object::Float(n % m),
        param => return Err(format!("Expected int or float, found {}", param)),
      },
      _ => return Err(format!("{} could not be divided", remainder)),
    }
  }

  Ok(remainder)
}

pub fn lt<I: Iterator<Item = Result<Object, String>>>(params: &mut I) -> Result<Object, String> {
  let mut result = true;

  let mut prev = params.next().unwrap()?;

  for param in params {
    let next = param?;

    result = match prev {
      Object::Integer(n) => match next {
        Object::Integer(m) => {
          prev = next;
          n < m
        }
        Object::Float(m) => {
          prev = next;
          n < m as i64
        }
        v => return Err(format!("Expected int or float, found {}", v)),
      },
      Object::Float(n) => match next {
        Object::Integer(m) => {
          prev = next;
          n < m as f64
        }
        Object::Float(m) => {
          prev = next;
          n < m
        }
        v => return Err(format!("Expected int or float, found {}", v)),
      },
      _ => return Err(format!("{} could not be compared", prev)),
    };

    if !result {
      break;
    }
  }

  Ok(Object::Bool(result))
}

pub fn gt<I: Iterator<Item = Result<Object, String>>>(params: &mut I) -> Result<Object, String> {
  let mut result = true;

  let mut prev = params.next().unwrap()?;

  for param in params {
    let next = param?;

    result = match prev {
      Object::Integer(n) => match next {
        Object::Integer(m) => {
          prev = next;
          n > m
        }
        Object::Float(m) => {
          n > {
            prev = next;
            m as i64
          }
        }
        v => return Err(format!("Expected int or float, found {}", v)),
      },
      Object::Float(n) => match next {
        Object::Integer(m) => {
          prev = next;
          n > m as f64
        }
        Object::Float(m) => {
          prev = next;
          n > m
        }
        v => return Err(format!("Expected int or float, found {}", v)),
      },
      _ => return Err(format!("{} could not be compared", prev)),
    };

    if !result {
      break;
    }
  }

  Ok(Object::Bool(result))
}

pub fn eq<I: Iterator<Item = Result<Object, String>>>(params: &mut I) -> Result<Object, String> {
  let mut result = true;

  let mut prev = params.next().unwrap()?;

  for param in params {
    let next = param?;

    let updated_prev = match (&prev, &next) {
      (Object::Integer(n), Object::Integer(m)) => n == m,
      (Object::Integer(n), Object::Float(m)) => *n == *m as i64,
      (Object::Integer(_), Object::Bool(_)) => true,
      (Object::Integer(_), _) => false,
      (Object::Float(n), Object::Integer(m)) => *n == *m as f64,
      (Object::Float(n), Object::Float(m)) => n == m,
      (Object::Float(_), Object::Bool(_)) => true,
      (Object::Float(_), _) => false,
      (Object::Bool(n), Object::Bool(m)) => n == m,
      (Object::Bool(n), Object::Void) => !n,
      (Object::Bool(b), Object::String(_)) => *b,
      (Object::Bool(_), _) => false,
      (Object::String(n), Object::String(m)) => n == m,
      (Object::String(_), Object::Bool(b)) => *b,
      (Object::String(_), _) => false,
      (Object::Void, Object::Void) => true,
      (Object::Void, Object::Bool(b)) => !b,
      (Object::Void, _) => false,
      (_, _) => return Err(format!("{} could not be compared", prev)),
    };

    if !updated_prev {
      result = false;
      break;
    }

    prev = next;
  }

  Ok(Object::Bool(result))
}

pub fn strict_eq<I: Iterator<Item = Result<Object, String>>>(
  params: &mut I,
) -> Result<Object, String> {
  let mut result = true;

  let mut prev = params.next().unwrap()?;

  for param in params {
    let next = param?;

    let updated_prev = match (&prev, &next) {
      (Object::Integer(n), Object::Integer(m)) => n == m,
      (Object::Integer(_), _) => false,
      (Object::Float(n), Object::Float(m)) => *n == *m,
      (Object::Float(_), _) => false,
      (Object::Bool(n), Object::Bool(m)) => n == m,
      (Object::Bool(_), _) => false,
      (Object::String(n), Object::String(m)) => n == m,
      (Object::String(_), _) => false,
      (Object::Void, Object::Void) => true,
      (Object::Void, _) => false,
      _ => return Err(format!("{} could not be compared", prev)),
    };

    if !updated_prev {
      result = false;
      break;
    }

    prev = next;
  }

  Ok(Object::Bool(result))
}

pub fn and<I: Iterator<Item = Result<Object, String>>>(params: &mut I) -> Result<Object, String> {
  let mut result = Object::Void;

  for param in params {
    let next = param?;

    result = match next {
      Object::Bool(b) => {
        if !b {
          return Ok(Object::Bool(false));
        }

        Object::Bool(true)
      }
      Object::Void => {
        return Ok(Object::Void);
      }
      v => v,
    }
  }

  Ok(result)
}

pub fn or<I: Iterator<Item = Result<Object, String>>>(params: &mut I) -> Result<Object, String> {
  let mut result = Object::Void;

  for param in params {
    let next = param?;

    result = match next {
      Object::Bool(b) => {
        if b {
          return Ok(Object::Bool(true));
        }

        Object::Bool(false)
      }
      Object::Void => Object::Void,
      v => return Ok(v),
    }
  }

  Ok(result)
}
