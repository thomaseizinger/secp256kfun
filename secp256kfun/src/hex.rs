//! Utility module for hex encoding and decoding
//XXX: is this actually meant to be this hard??
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::String;
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::string::String;
#[cfg(feature = "std")]
use std::vec::Vec;

use core::fmt;
/// Error representing a failed conversion from hex into the bytes for the target type.
#[derive(Debug, Clone, PartialEq)]
pub enum HexError {
    /// The string was not a valid hex string.
    InvalidHex,
    /// The string was not the right length for the target type.
    InvalidLength,
    /// The bytes did not encode a valid value for the target type.
    InvalidEncoding,
}

impl fmt::Display for HexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use HexError::*;
        match self {
            InvalidHex => write!(f, "invalid hex string"),
            InvalidLength => write!(f, "hex string had an invalid (odd) length"),
            InvalidEncoding => write!(f, "hex value did not encode the expected type"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HexError {}

#[doc(hidden)]
pub fn hex_val(c: u8) -> Result<u8, HexError> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(HexError::InvalidHex),
    }
}

#[cfg(feature = "alloc")]
/// Encode some bytes as a hex String.
///
/// # Examples
/// ```
/// use secp256kfun::{ G, hex};
/// let G_hex = hex::encode(G.to_bytes().as_ref());
/// assert_eq!(G_hex, "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
pub fn encode(bytes: &[u8]) -> String {
    use core::fmt::Write;
    let mut hex = String::new();
    for byte in bytes {
        write!(hex, "{:02x}", byte).unwrap()
    }
    hex
}

/// Decode some hex bytes into a `Vec<u8>`.
///
/// # Examples
/// ```
/// use secp256kfun::{ G, hex};
/// let G_bytes =  hex::decode("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798").unwrap();
/// assert_eq!(&G_bytes[..], G.to_bytes().as_ref());
#[cfg(feature = "alloc")]
pub fn decode(hex: &str) -> Result<Vec<u8>, HexError> {
    if (hex.len() % 2) != 0 {
        return Err(HexError::InvalidLength);
    }
    let mut bytes = Vec::with_capacity(hex.len() * 2);

    for hex_byte in hex.as_bytes().chunks(2) {
        bytes.push(hex_val(hex_byte[0])? << 4 | hex_val(hex_byte[1])?)
    }

    Ok(bytes)
}
