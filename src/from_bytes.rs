use crate::*;

pub trait FromBytes: Sized {
    fn from_bytes(input: Vec<u8>) -> Result<Self, Error>;
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
