use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

mod decode;
mod encode;

/*
- resp frame
    - simple string: "+OK\r\n"
    - error: "-Error message\r\n"
    - bulk error: "!<length>\r\n<error>\r\n"
    - integer: ":[<+|->]<value>\r\n"
    - bulk string: "$<length>\r\n<data>\r\n"
    - null bulk string: "$-1\r\n"
    - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
        - "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
    - null array: "*-1\r\n"
    - null: "_\r\n"
    - boolean: "#<t|f>\r\n"
    - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
    - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
    - set: "~<number-of-elements>\r\n<element-1>...<element-n>"
    - ...
- enum RespFrame {}
- trait RespEncode / RespDecode (enum dispatch)
- bytes trait
*/

pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(data: &[u8]) -> Result<RespFrame, String>;
}

pub enum RespFrame {
    SimpleString(SimpleString),
    SimpleError(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    Array(Array),
    Null(RespNull),
    NullArray(RespNullArray),
    NullBulkString(RespNullBulkString),
    Boolean(bool),
    Double(f64),
    BulkError(BulkError),

    Map(Map),
    Set(Set),
}

impl RespEncode for RespFrame {
    fn encode(self) -> Vec<u8> {
        todo!()
    }
}

pub struct SimpleString(String);
pub struct SimpleError(String);

pub struct BulkString(Vec<u8>);

pub struct Array(Vec<RespFrame>);

pub struct BulkError(String);

pub struct RespNull;

pub struct RespNullBulkString;

pub struct RespNullArray;

pub struct Map(HashMap<String, RespFrame>);

pub struct Set(HashSet<RespFrame>);

// impl Deref and DerefMut for SimpleString, SimpleError, BulkString, Array, BulkError, Map, Set
impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Array {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BulkError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Map {
    type Target = HashMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Set {
    type Target = HashSet<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SimpleString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for SimpleError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for BulkString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for Array {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for BulkError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for Set {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// impl From for SimpleString, SimpleError, BulkString, Array, BulkError, Map, Set
impl From<String> for SimpleString {
    fn from(s: String) -> Self {
        SimpleString(s)
    }
}

impl From<&str> for SimpleString {
    fn from(s: &str) -> Self {
        SimpleString(s.to_string())
    }
}

impl From<String> for SimpleError {
    fn from(s: String) -> Self {
        SimpleError(s)
    }
}

impl From<&str> for SimpleError {
    fn from(s: &str) -> Self {
        SimpleError(s.to_string())
    }
}

impl From<Vec<u8>> for BulkString {
    fn from(v: Vec<u8>) -> Self {
        BulkString(v)
    }
}

impl From<&[u8]> for BulkString {
    fn from(v: &[u8]) -> Self {
        BulkString(v.to_vec())
    }
}

impl From<Vec<RespFrame>> for Array {
    fn from(v: Vec<RespFrame>) -> Self {
        Array(v)
    }
}

impl From<Vec<u8>> for BulkError {
    fn from(v: Vec<u8>) -> Self {
        BulkError(String::from_utf8(v).unwrap())
    }
}

impl From<&[u8]> for BulkError {
    fn from(v: &[u8]) -> Self {
        BulkError(String::from_utf8(v.to_vec()).unwrap())
    }
}

impl From<HashMap<String, RespFrame>> for Map {
    fn from(v: HashMap<String, RespFrame>) -> Self {
        Map(v)
    }
}

impl From<HashSet<RespFrame>> for Set {
    fn from(v: HashSet<RespFrame>) -> Self {
        Set(v)
    }
}
