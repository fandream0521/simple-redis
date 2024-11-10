// impl RespEncode for all types
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
*/

use super::*;

// simple string: "+OK\r\n"
impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

// error: "-Error message\r\n"
impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

// integer: ":[<+|->]<value>\r\n"
impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "-" } else { "+" };
        format!(":{}{}\r\n", sign, self.abs()).into_bytes()
    }
}

// bulk string: "$<length>\r\n<data>\r\n"
impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        format!(
            "${}\r\n{}\r\n",
            self.len(),
            String::from_utf8(self.0).unwrap()
        )
        .into_bytes()
    }
}

// bulk error: "!<length>\r\n<error>\r\n"
impl RespEncode for BulkError {
    fn encode(self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(self.len() + 16);
        buff.extend_from_slice(&format!("!{}\r\n", self.len()).into_bytes());
        buff.extend_from_slice(self.as_bytes());
        buff.extend_from_slice(b"\r\n");
        buff
    }
}

// array: "*<number-of-elements>\r\n<element-1>...<element-n>"
impl RespEncode for Array {
    fn encode(self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(16);
        buff.extend_from_slice(&format!("*{}\r\n", self.0.len()).into_bytes());
        for frame in self.0 {
            buff.extend_from_slice(&frame.encode());
        }
        buff
    }
}

// null bulk string: "$-1\r\n"
impl RespEncode for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        "$-1\r\n".as_bytes().to_vec()
    }
}

// null array: "*-1\r\n"
impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        "*-1\r\n".as_bytes().to_vec()
    }
}

// null: "_\r\n"
impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        "_\r\n".as_bytes().to_vec()
    }
}

// boolean: "#<t|f>\r\n"
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        format!("#{}\r\n", if self { "t" } else { "f" }).into_bytes()
    }
}

// double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let ret = if self.abs() > 1e+8 {
            format!(",{:.0e}\r\n", self)
        } else {
            format!(",{}\r\n", self)
        };
        ret.into_bytes()
    }
}

// map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
impl RespEncode for Map {
    fn encode(self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(16);
        buff.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());
        for (key, value) in self.0 {
            buff.extend_from_slice(&SimpleString(key).encode());
            buff.extend_from_slice(&value.encode());
        }
        buff
    }
}

// set: "~<number-of-elements>\r\n<element-1>...<element-n>"
impl RespEncode for Set {
    fn encode(self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(16);
        buff.extend_from_slice(&format!("~{}\r\n", self.len()).into_bytes());
        for frame in self.0 {
            buff.extend_from_slice(&frame.encode());
        }
        buff
    }
}
