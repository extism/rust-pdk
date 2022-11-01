use crate::*;

pub trait Input: Sized {
    fn input(input: Vec<u8>) -> Result<Self, Error>;
}

impl Input for Vec<u8> {
    fn input(input: Vec<u8>) -> Result<Self, Error> {
        Ok(input)
    }
}

impl Input for String {
    fn input(input: Vec<u8>) -> Result<Self, Error> {
        let s = String::from_utf8(input)?;
        Ok(s)
    }
}

impl Input for json::Value {
    fn input(input: Vec<u8>) -> Result<Self, Error> {
        let j = serde_json::from_slice(&input)?;
        Ok(j)
    }
}
