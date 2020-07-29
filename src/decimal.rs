use rust_decimal::prelude::*;
use std::ops::*;
use std::convert::{TryFrom};
use std::any::TypeId;
use core::fmt;
use crate::decimal::xdecimalErr::NoneResult;


/// Decimal error type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum xdecimalErr {
    /// unsupported input type.
    UnsupportedFromType { from_type: TypeId},
    NoneResult,
}

pub struct xdecimal {
    ir: Decimal,
}


impl std::error::Error for xdecimalErr {}
impl std::convert::From<&str> for xdecimalErr {
    fn from(value: &str) -> xdecimalErr {
        NoneResult
    }
}

impl fmt::Display for xdecimalErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            xdecimalErr::UnsupportedFromType { from_type } => {
                write!(f, "Unsupported decimal from type {:?}", from_type)
            },
            xdecimalErr::NoneResult => {
                write!(f, "none result")
            }
        }
    }
}

impl TryFrom<i64> for xdecimal {
    type Error = xdecimalErr;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(xdecimal {ir:Decimal::from_i64(value).ok_or(NoneResult)?})
    }
}


impl TryFrom<f64> for xdecimal {
    type Error = xdecimalErr;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(xdecimal {ir:Decimal::from_f64(value).ok_or(NoneResult)?})
    }
}

impl TryFrom<&str> for xdecimal {
    type Error = rust_decimal::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(xdecimal {ir:Decimal::from_str(value)?})
    }
}

impl xdecimal {
    fn from_str(value: &str) -> Result<xdecimal, rust_decimal::Error> {
        let res = xdecimal {
            ir:  Decimal::from_str(value)?,
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

impl Add for xdecimal {
    type Output = xdecimal;
    fn add(self, other: xdecimal) -> xdecimal {
        xdecimal {ir: self.ir + other.ir}
    }
}

impl Sub for xdecimal {
    type Output = xdecimal;
    fn sub(self, other: xdecimal) -> xdecimal {
        xdecimal {ir: self.ir - other.ir}
    }
}

impl Mul for xdecimal {
    type Output = xdecimal;
    fn mul(self, other: xdecimal) -> xdecimal {
        xdecimal {ir: self.ir * other.ir}
    }
}

impl Div for xdecimal {
    type Output = xdecimal;
    fn div(self, other: xdecimal) -> xdecimal {
        xdecimal {ir: self.ir / other.ir}
    }
}

impl PartialEq for xdecimal {
    fn eq(&self, other: &xdecimal) -> bool {
        self.ir == other.ir
    }
    fn ne(&self, other: &xdecimal) -> bool {
        self.ir != other.ir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xdecimal_from_str_test() {
        let left = xdecimal::try_from("1.23").unwrap();
        assert_eq!(left.to_string(), "1.23");
        let right = xdecimal::try_from("2.34").unwrap();
        assert_eq!(right.to_string(), "2.34");
    }

    #[test]
    fn xdecimal_to_f64_test() {
        let left = xdecimal::try_from("1.23").unwrap();
        assert_eq!(left.to_f64(), 1.23);
        let right = xdecimal::try_from("2.34").unwrap();
        assert_eq!(right.to_f64(), 2.34);
    }

    #[test]
    fn xdecimal_add_test() {
        let left = xdecimal::try_from("1.23").unwrap();
        let right = xdecimal::try_from("2.34").unwrap();
        let sum = left + right;
        assert_eq!(sum.to_string(), "3.57");
    }
}