use crate::*;

use base64::Engine;

pub trait FromBytes<T = Self>: Sized {
    fn from_bytes(input: Vec<u8>) -> Result<T, Error>;
}

impl FromBytes for () {
    fn from_bytes(_input: Vec<u8>) -> Result<Self, Error> {
        Ok(())
    }
}

impl FromBytes for Vec<u8> {
    fn from_bytes(input: Vec<u8>) -> Result<Self, Error> {
        Ok(input)
    }
}

impl FromBytes for String {
    fn from_bytes(input: Vec<u8>) -> Result<Self, Error> {
        let s = String::from_utf8(input)?;
        Ok(s)
    }
}

impl FromBytes for json::Value {
    fn from_bytes(input: Vec<u8>) -> Result<Self, Error> {
        let j = serde_json::from_slice(&input)?;
        Ok(j)
    }
}

impl FromBytes<Vec<u8>> for Base64 {
    fn from_bytes(input: Vec<u8>) -> Result<Vec<u8>, Error> {
        Ok(base64::engine::general_purpose::STANDARD.decode(input)?)
    }
}
