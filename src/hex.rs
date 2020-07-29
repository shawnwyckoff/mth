use core::fmt;

const HEX_TABLE_LOWER:&[u8] = b"0123456789abcdef";
const HEX_TABLE_UPPER:&[u8] = b"0123456789ABCDEF";

/// Hex codec error type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HexErr {
    /// A hex string's length must be even, as two digits correspond to one byte.
    OddLen,

    /// Hex string must decoded into a fixed size container which has hex string's length / 2.
    InvalidStrLen,

    /// Invalid character which is not in valid ones: `0...9`, `a...f` or `A...F`.
    InvalidHexChar { c: char, index: usize },
}

/// Hex encoder trait
pub trait HexEnc {
    /// Encode hex number to hex string
    /// Example: encode hex u8 slice [0x18, 0x19] to str "1819" which equals to u8 slice [0x31, 0x38, 0x31, 0x39]
    fn u_hex_encode(&self, upper:bool) -> String;
}

/// Hex decoder trait
pub trait HexDec {
    /// Decode hex string to hex number
    /// Example: decode str "1819" to u8 slice [0x18, 0x19]
    fn u_hex_decode(&self) -> Result<Vec<u8>, HexErr>;
}



impl std::error::Error for HexErr {}

impl fmt::Display for HexErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HexErr::OddLen => write!(f, "Odd length of string to decode"),
            HexErr::InvalidStrLen => write!(f, "Invalid string length"),
            HexErr::InvalidHexChar { c, index } => {
                write!(f, "Invalid char {:?} at index {}", c, index)
            }
        }
    }
}


impl HexEnc for &[u8] {
    fn u_hex_encode(&self, upper:bool) -> String {
        let mut dst = vec![0; self.len() * 2];
        _hex_encode( self.as_ref(), &mut dst, upper);
        return String::from_utf8(dst).unwrap();
    }
}

impl HexDec for str {
    fn u_hex_decode(&self) -> Result<Vec<u8>, HexErr> {
        let mut dst = vec![0; self.len() / 2];
        _hex_decode( self.as_ref(), &mut dst)?;
        return Ok(dst.to_vec());
    }
}



#[inline]
pub fn _hex_encode(src: &[u8], dst: &mut [u8], upper: bool) {
    let mut hex_table = HEX_TABLE_LOWER;
    if upper {
        hex_table = HEX_TABLE_UPPER
    }
    src.iter().fold(0, |k, v| {
        dst[k] = hex_table[*v as usize >> 4];
        dst[k + 1] = hex_table[*v as usize & 0x0f];
        k + 2
    });
}

// Hex char to u8 number
#[inline]
fn _hex_char_to_u8(c: u8, idx: usize) -> Result<u8, HexErr> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(HexErr::InvalidHexChar {
            c: c as char,
            index: idx,
        }),
    }
}

#[inline]
pub fn _hex_decode(src: &[u8], dst: &mut [u8]) -> Result<(), HexErr> {
    if src.len() % 2 != 0 {
        return Err(HexErr::OddLen);
    }
    if src.len() / 2 != dst.len() {
        return Err(HexErr::InvalidStrLen);
    }

    for i in 0..dst.len() {
        dst[i] = _hex_char_to_u8(src[2 * i], 2 * i)? << 4 | _hex_char_to_u8(src[2 * i + 1], 2 * i + 1)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u_hex_encode_test() {
        let src = [0x18, 0x19];
        let s = src.as_ref().u_hex_encode(true);
        assert_eq!(s, "1819");
    }

    #[test]
    fn u_hex_decode_test() {
        let src = "1819";
        let dst = src.u_hex_decode();
        assert_eq!(dst, Ok(vec![0x18, 0x19]));
    }
}