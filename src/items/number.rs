use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};
use std::i64;

use serde::{Serialize, Serializer};

#[derive(Clone, PartialEq, PartialOrd)]
pub struct Number {
    n: N,
}

impl fmt::Display for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.n {
            N::PosInt(i) => Display::fmt(&i, formatter),
            N::NegInt(i) => Display::fmt(&i, formatter),
            N::Float(f) if f.is_nan() => formatter.write_str(".nan"),
            N::Float(f) if f.is_infinite() => {
                if f.is_sign_negative() {
                    formatter.write_str("-.inf")
                } else {
                    formatter.write_str(".inf")
                }
            }
            N::Float(f) => Display::fmt(&f, formatter),
        }
    }
}

impl Debug for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.n, formatter)
    }
}

#[derive(Copy, Clone, Debug)]
enum N {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

impl Number {
    #[inline]
    pub fn from_str(n: &str) -> Option<Number> {
        match n.parse::<u64>() {
            Ok(num) => Some(Number { n: N::PosInt(num) }),
            _ => match n.parse::<i64>() {
                Ok(num) => Some(Number { n: N::NegInt(num) }),
                _ => match n.parse::<f64>() {
                    Ok(num) => Some(Number { n: N::Float(num) }),
                    _ => None,
                },
            },
        }
    }
}

impl PartialEq for N {
    fn eq(&self, other: &N) -> bool {
        match (*self, *other) {
            (N::PosInt(a), N::PosInt(b)) => a == b,
            (N::NegInt(a), N::NegInt(b)) => a == b,
            (N::Float(a), N::Float(b)) => {
                if a.is_nan() && b.is_nan() {
                    true
                } else {
                    a == b
                }
            }
            _ => false,
        }
    }
}

impl PartialOrd for N {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self, *other) {
            (N::Float(a), N::Float(b)) => {
                if a.is_nan() && b.is_nan() {
                    Some(Ordering::Equal)
                } else {
                    a.partial_cmp(&b)
                }
            }
            _ => Some(self.total_cmp(other)),
        }
    }
}

impl N {
    fn total_cmp(&self, other: &Self) -> Ordering {
        match (*self, *other) {
            (N::PosInt(a), N::PosInt(b)) => a.cmp(&b),
            (N::NegInt(a), N::NegInt(b)) => a.cmp(&b),
            (N::NegInt(_), N::PosInt(_)) => Ordering::Less,
            (N::PosInt(_), N::NegInt(_)) => Ordering::Greater,
            (N::Float(a), N::Float(b)) => a.partial_cmp(&b).unwrap_or_else(|| {
                if !a.is_nan() {
                    Ordering::Less
                } else if !b.is_nan() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }),
            (_, N::Float(_)) => Ordering::Less,
            (N::Float(_), _) => Ordering::Greater,
        }
    }
}

impl From<f64> for Number {
    #[inline]
    fn from(f: f64) -> Self {
        let n = { N::Float(f) };
        Number { n }
    }
}

impl Serialize for Number {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.n {
            N::PosInt(u) => serializer.serialize_u64(u),
            N::NegInt(i) => serializer.serialize_i64(i),
            N::Float(f) => serializer.serialize_f64(f),
        }
    }
}

macro_rules! impl_from_unsigned {
  (
      $($ty:ty),*
  ) => {
      $(
          impl From<$ty> for Number {
              #[inline]
              fn from(u: $ty) -> Self {
                  let n = {
                      N::PosInt(u as u64)
                  };
                  Number { n }
              }
          }
      )*
  };
}

macro_rules! impl_from_signed {
  (
      $($ty:ty),*
  ) => {
      $(
          impl From<$ty> for Number {
              #[inline]
              fn from(i: $ty) -> Self {
                  let n = {
                      if i < 0 {
                          N::NegInt(i as i64)
                      } else {
                          N::PosInt(i as u64)
                      }
                  };
                  Number { n }
              }
          }
      )*
  };
}

impl_from_unsigned!(u8, u16, u32, u64, usize);
impl_from_signed!(i8, i16, i32, i64, isize);

// UNIT TESTS /////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Number;

    #[test]
    fn detect_numbers_from_str() {
        assert_eq!(Number::from_str("123"), Some(123u64.into()));
        assert_eq!(
            Number::from_str("18446744073709551615"),
            Some(18446744073709551615u64.into())
        );
        assert_eq!(
            Number::from_str("18446744073709551615118446744073709551615"),
            Some(1.8446744073709552e40f64.into())
        );

        assert_eq!(Number::from_str("-123"), Some((-123).into()));
        assert_eq!(
            Number::from_str("-9223372036854775807"),
            Some((-9223372036854775807i64).into())
        );

        assert_eq!(Number::from_str("123.456"), Some((123.456f64).into()));
        assert_eq!(
            Number::from_str("18446744073709551615.456"),
            Some((18446744073709551615.456f64).into())
        );
        assert_eq!(
            Number::from_str("123e10"),
            Some((1230000000000.0f64).into())
        );

        assert_eq!(Number::from_str("a123"), None);
        assert_eq!(Number::from_str("123.a"), None);
        assert_eq!(Number::from_str("123e"), None);
        assert_eq!(Number::from_str("hello"), None);
    }
}
