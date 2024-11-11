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
        let ret = if self.abs() > 1e+8 || self.abs() < 1e-8 {
            format!(",{:e}\r\n", self)
        } else {
            format!(",{}\r\n", self)
        };
        ret.into_bytes()
    }
}

// map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
// only support simple string key
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
            buff.extend_from_slice(&frame.1.encode());
        }
        buff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_string_encode() {
        let frame = SimpleString("OK".to_string());
        assert_eq!(frame.encode(), b"+OK\r\n");
    }

    #[test]
    fn test_simple_error_encode() {
        let frame = SimpleError("Error message".to_string());
        assert_eq!(frame.encode(), b"-Error message\r\n");
    }

    #[test]
    fn test_integer_encode() {
        let frame = 123;
        assert_eq!(frame.encode(), b":+123\r\n");

        let frame = -123;
        assert_eq!(frame.encode(), b":-123\r\n");
    }

    #[test]
    fn test_bulk_string_encode() {
        let frame = BulkString(b"hello".to_vec());
        assert_eq!(frame.encode(), b"$5\r\nhello\r\n");
    }

    #[test]
    fn test_bulk_error_encode() {
        let frame = BulkError("Error message".to_string());
        assert_eq!(frame.encode(), b"!13\r\nError message\r\n");
    }

    #[test]
    fn test_array_encode() {
        let frame = Array(vec![
            RespFrame::BulkString(BulkString(b"get".to_vec())),
            RespFrame::BulkString(BulkString(b"hello".to_vec())),
        ]);
        assert_eq!(frame.encode(), b"*2\r\n$3\r\nget\r\n$5\r\nhello\r\n");
    }

    #[test]
    fn test_null_bulk_string_encode() {
        let frame = RespNullBulkString;
        assert_eq!(frame.encode(), b"$-1\r\n");
    }

    #[test]
    fn test_null_array_encode() {
        let frame = RespNullArray;
        assert_eq!(frame.encode(), b"*-1\r\n");
    }

    #[test]
    fn test_null_encode() {
        let frame = RespNull;
        assert_eq!(frame.encode(), b"_\r\n");
    }

    #[test]
    fn test_boolean_encode() {
        let frame = true;
        assert_eq!(frame.encode(), b"#t\r\n");

        let frame = false;
        assert_eq!(frame.encode(), b"#f\r\n");
    }

    #[test]
    fn test_double_encode() {
        let frame = 123.456;
        assert_eq!(frame.encode(), b",123.456\r\n");

        let frame = 123.0;
        assert_eq!(frame.encode(), b",123\r\n");

        let frame = 1.23456e+8;
        assert_eq!(frame.encode(), b",1.23456e8\r\n");

        let frame = 1.23456e-9;
        assert_eq!(frame.encode(), b",1.23456e-9\r\n");
    }

    #[test]
    fn test_map_encode() {
        let mut map = BTreeMap::new();
        map.insert(
            "key1".to_string(),
            RespFrame::BulkString(BulkString(b"value1".to_vec())),
        );
        map.insert(
            "key2".to_string(),
            RespFrame::BulkString(BulkString(b"value2".to_vec())),
        );
        let frame = Map(map);
        assert_eq!(
            frame.encode(),
            b"%2\r\n+key1\r\n$6\r\nvalue1\r\n+key2\r\n$6\r\nvalue2\r\n"
        );
    }

    #[test]
    fn test_set_encode() {
        let mut map = BTreeMap::new();
        map.insert(
            "key1".to_string(),
            RespFrame::BulkString(BulkString(b"value1".to_vec())),
        );
        map.insert(
            "key2".to_string(),
            RespFrame::BulkString(BulkString(b"value2".to_vec())),
        );
        let frame = Set(map);
        let encoded = frame.encode();
        println!("{:?}", String::from_utf8_lossy(&encoded));
        assert_eq!(encoded, b"~2\r\n$6\r\nvalue1\r\n$6\r\nvalue2\r\n");
    }
}
