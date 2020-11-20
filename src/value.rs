use crate::serialize::resp_encode;
use crate::Result;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Value {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<Self>),
    Null,
}

impl Value {
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_simple_string(&self) -> bool {
        matches!(self, Self::SimpleString(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn is_bulk_string(&self) -> bool {
        matches!(self, Self::BulkString(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn simple_string(value: &str) -> Self {
        Self::SimpleString(value.to_string())
    }

    pub fn error(value: &str) -> Self {
        Self::Error(value.to_string())
    }

    pub fn integer(value: i64) -> Self {
        Self::Integer(value)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        resp_encode(self)
    }
}
