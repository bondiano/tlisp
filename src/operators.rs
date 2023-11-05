use crate::object::Object;

pub fn sum(params: Vec<Object>) -> Result<Object, String> {
  let mut sum: Object = params[0].clone();

  let rest_params = &params[1..];

  for param in rest_params {
    sum = match sum {
      Object::Integer(n) => match param {
        Object::Integer(m) => Object::Integer(m + n),
        Object::Float(m) => Object::Float(m + n as f64),
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::Float(n) => match param {
        Object::Integer(m) => Object::Float(*m as f64 + n),
        Object::Float(m) => Object::Float(m + n),
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::String(s) => match param {
        Object::String(t) => Object::String(format!("{}{}", s, t)),
        _ => return Err(format!("Expected string, found {}", param)),
      },
      _ => return Err(format!("{} could not be added", sum)),
    }
  }

  Ok(sum)
}

pub fn sub(params: Vec<Object>) -> Result<Object, String> {
  let mut diff: Object = params[0].clone();

  if params.len() == 1 {
    diff = match diff {
      Object::Integer(n) => Object::Integer(-n),
      Object::Float(n) => Object::Float(-n),
      _ => return Err(format!("Expected int or float, found {}", diff)),
    };

    return Ok(diff);
  }

  let rest_params = &params[1..];

  for param in rest_params {
    diff = match diff {
      Object::Integer(n) => match param {
        Object::Integer(m) => Object::Integer(n - m),
        Object::Float(m) => Object::Float(n as f64 - m),
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::Float(n) => match param {
        Object::Integer(m) => Object::Float(n - *m as f64),
        Object::Float(m) => Object::Float(n - m),
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      _ => return Err(format!("{} could not be subtracted", diff)),
    }
  }

  Ok(diff)
}

pub fn mult(params: Vec<Object>) -> Result<Object, String> {
  let mut product: Object = params[0].clone();

  let rest_params = &params[1..];

  for param in rest_params {
    product = match product {
      Object::Integer(n) => match param {
        Object::Integer(m) => Object::Integer(m * n),
        Object::Float(m) => Object::Float(m * n as f64),
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::Float(n) => match param {
        Object::Integer(m) => Object::Float(*m as f64 * n),
        Object::Float(m) => Object::Float(m * n),
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      _ => return Err(format!("{} could not be multiplied", product)),
    }
  }

  Ok(product)
}

pub fn div(params: Vec<Object>) -> Result<Object, String> {
  let mut quotient: Object = params[0].clone();

  let rest_params = &params[1..];

  for param in rest_params {
    quotient = match quotient {
      Object::Integer(n) => match param {
        Object::Integer(m) => Object::Integer(n / m),
        Object::Float(m) => Object::Float(n as f64 / m),
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::Float(n) => match param {
        Object::Integer(m) => Object::Float(n / *m as f64),
        Object::Float(m) => Object::Float(n / m),
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      _ => return Err(format!("{} could not be divided", quotient)),
    }
  }

  Ok(quotient)
}

pub fn mod_(params: Vec<Object>) -> Result<Object, String> {
  let mut remainder: Object = params[0].clone();

  let rest_params = &params[1..];

  for param in rest_params {
    remainder = match remainder {
      Object::Integer(n) => match param {
        Object::Integer(m) => Object::Integer(n % m),
        Object::Float(m) => Object::Float(n as f64 % m),
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::Float(n) => match param {
        Object::Integer(m) => Object::Float(n % *m as f64),
        Object::Float(m) => Object::Float(n % m),
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      _ => return Err(format!("{} could not be divided", remainder)),
    }
  }

  Ok(remainder)
}

pub fn lt(params: Vec<Object>) -> Result<Object, String> {
  let mut result = true;

  let mut prev = params[0].clone();

  let rest_params = &params[1..];

  for param in rest_params {
    result = match prev {
      Object::Integer(n) => match param {
        Object::Integer(m) => n < *m,
        Object::Float(m) => n < *m as i64,
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::Float(n) => match param {
        Object::Integer(m) => n < *m as f64,
        Object::Float(m) => n < *m,
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      _ => return Err(format!("{} could not be compared", prev)),
    };

    if !result {
      break;
    }

    prev = param.clone();
  }

  Ok(Object::Bool(result))
}

pub fn gt(params: Vec<Object>) -> Result<Object, String> {
  let mut result = true;

  let mut prev = params[0].clone();

  let rest_params = &params[1..];

  for param in rest_params {
    result = match prev {
      Object::Integer(n) => match param {
        Object::Integer(m) => n > *m,
        Object::Float(m) => n > *m as i64,
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      Object::Float(n) => match param {
        Object::Integer(m) => n > *m as f64,
        Object::Float(m) => n > *m,
        _ => return Err(format!("Expected int or float, found {}", param)),
      },
      _ => return Err(format!("{} could not be compared", prev)),
    };

    if !result {
      break;
    }

    prev = param.clone();
  }

  Ok(Object::Bool(result))
}

pub fn eq(params: Vec<Object>) -> Result<Object, String> {
  let mut result = true;

  let mut prev = params[0].clone();

  let rest_params = &params[1..];

  for param in rest_params {
    result = match prev {
      Object::Integer(n) => match param {
        Object::Integer(m) => n == *m,
        Object::Float(m) => n == *m as i64,
        Object::Bool(_) => true,
        _ => false,
      },
      Object::Float(n) => match param {
        Object::Integer(m) => n == *m as f64,
        Object::Float(m) => n == *m,
        Object::Bool(_) => true,
        _ => false,
      },
      Object::Bool(n) => match param {
        Object::Bool(m) => n == *m,
        Object::Void => !n,
        _ => true,
      },
      Object::String(n) => match param {
        Object::String(m) => n == *m,
        Object::Bool(b) => *b,
        _ => false,
      },
      Object::Void => match param {
        Object::Void => true,
        Object::Bool(b) => !b,
        _ => false,
      },
      _ => return Err(format!("{} could not be compared", prev)),
    };

    if !result {
      break;
    }

    prev = param.clone();
  }

  Ok(Object::Bool(result))
}

pub fn strict_eq(params: Vec<Object>) -> Result<Object, String> {
  let mut result = true;

  let mut prev = params[0].clone();

  let rest_params = &params[1..];

  for param in rest_params {
    result = match prev {
      Object::Integer(n) => match param {
        Object::Integer(m) => n == *m,
        _ => false,
      },
      Object::Float(n) => match param {
        Object::Float(m) => n == *m,
        _ => false,
      },
      Object::Bool(n) => match param {
        Object::Bool(m) => n == *m,
        _ => false,
      },
      Object::String(n) => match param {
        Object::String(m) => n == *m,
        _ => false,
      },
      Object::Void => match param {
        Object::Void => true,
        _ => false,
      },
      _ => return Err(format!("{} could not be compared", prev)),
    };

    if !result {
      break;
    }

    prev = param.clone();
  }

  Ok(Object::Bool(result))
}
