use crate::number::decimal::XdecimalErr::NoneResult;
use core::fmt;
use rust_decimal::prelude::*;
use std::any::TypeId;
use std::convert::TryFrom;
use std::ops::*;

/// Decimal error type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum XdecimalErr {
    /// unsupported input type.
    UnsupportedFromType {
        from_type: TypeId,
    },
    NoneResult,
}

pub struct Xdecimal {
    ir: Decimal,
}

impl std::error::Error for XdecimalErr {}
impl std::convert::From<&str> for XdecimalErr {
    fn from(value: &str) -> XdecimalErr {
        NoneResult
    }
}

impl fmt::Display for XdecimalErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XdecimalErr::UnsupportedFromType { from_type } => {
                write!(f, "Unsupported decimal from type {:?}", from_type)
            }
            XdecimalErr::NoneResult => write!(f, "none result"),
        }
    }
}

impl TryFrom<i64> for Xdecimal {
    type Error = XdecimalErr;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Xdecimal {
            ir: Decimal::from_i64(value).ok_or(NoneResult)?,
        })
    }
}

impl TryFrom<f64> for Xdecimal {
    type Error = XdecimalErr;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Xdecimal {
            ir: Decimal::from_f64(value).ok_or(NoneResult)?,
        })
    }
}

impl TryFrom<&str> for Xdecimal {
    type Error = rust_decimal::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Xdecimal {
            ir: Decimal::from_str(value)?,
        })
    }
}

impl Xdecimal {
    fn from_str(value: &str) -> Result<Xdecimal, rust_decimal::Error> {
        let res = Xdecimal {
            ir: Decimal::from_str(value)?,
        };
        return Ok(res);
    }

    fn to_f64(&self) -> f64 {
        return self.ir.to_f64().unwrap();
    }

    fn to_string(&self) -> String {
        return self.ir.to_string();
    }
}

impl Add for Xdecimal {
    type Output = Xdecimal;
    fn add(self, other: Xdecimal) -> Xdecimal {
        Xdecimal {
            ir: self.ir + other.ir,
        }
    }
}

impl Sub for Xdecimal {
    type Output = Xdecimal;
    fn sub(self, other: Xdecimal) -> Xdecimal {
        Xdecimal {
            ir: self.ir - other.ir,
        }
    }
}

impl Mul for Xdecimal {
    type Output = Xdecimal;
    fn mul(self, other: Xdecimal) -> Xdecimal {
        Xdecimal {
            ir: self.ir * other.ir,
        }
    }
}

impl Div for Xdecimal {
    type Output = Xdecimal;
    fn div(self, other: Xdecimal) -> Xdecimal {
        Xdecimal {
            ir: self.ir / other.ir,
        }
    }
}

impl PartialEq for Xdecimal {
    fn eq(&self, other: &Xdecimal) -> bool {
        self.ir == other.ir
    }
    fn ne(&self, other: &Xdecimal) -> bool {
        self.ir != other.ir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xdecimal_from_str_test() {
        let left = Xdecimal::try_from("1.23").unwrap();
        assert_eq!(left.to_string(), "1.23");
        let right = Xdecimal::try_from("2.34").unwrap();
        assert_eq!(right.to_string(), "2.34");
    }

    #[test]
    fn xdecimal_to_f64_test() {
        let left = Xdecimal::try_from("1.23").unwrap();
        assert_eq!(left.to_f64(), 1.23);
        let right = Xdecimal::try_from("2.34").unwrap();
        assert_eq!(right.to_f64(), 2.34);
    }

    #[test]
    fn xdecimal_add_test() {
        let left = Xdecimal::try_from("1.23").unwrap();
        let right = Xdecimal::try_from("2.34").unwrap();
        let sum = left + right;
        assert_eq!(sum.to_string(), "3.57");
    }
}
