use std::ops::Deref;

use bytes::{Buf, BytesMut};

use crate::{RespDecode, RespEncode, RespError};

use super::{extract_fixed_data, parse_length, CRLF_LEN};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct BulkString(pub(crate) Vec<u8>);

impl BulkString {
    const NULL: &'static str = "$-1\r\n";
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        BulkString(s.into())
    }
}

// - null bulk string: "$-1\r\n"
// - bulk string: "$<length>\r\n<data>\r\n"
impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        if self.is_empty() {
            return Self::NULL.as_bytes().to_vec();
        }
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("${}\r\n", self.len()).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

/// - null bulk string: "$-1\r\n"
// - bulk string: "$<length>\r\n<data>\r\n"
impl RespDecode for BulkString {
    const PREFIX: &'static str = "$";
    fn decode(buf: &mut BytesMut) -> anyhow::Result<Self, RespError> {
        if extract_fixed_data(buf, Self::NULL, "NullBulkString").is_ok() {
            return Ok(BulkString::new([]));
        }

        let (end, len) = parse_length(buf, Self::PREFIX)?;
        let remained = &buf[end + CRLF_LEN..];
        if remained.len() < len + CRLF_LEN {
            return Err(RespError::NotComplete);
        }

        // $length\r\n 部分给移除
        buf.advance(end + CRLF_LEN);

        // 把 buf 中的 <数据\r\n> 部分给移除
        let data = buf.split_to(len + CRLF_LEN);
        Ok(BulkString::new(data[0..len].to_vec()))
    }

    fn expect_length(buf: &[u8]) -> anyhow::Result<usize, RespError> {
        if buf.starts_with(Self::NULL.as_bytes()) {
            return Ok(Self::NULL.len());
        }

        let (end, len) = parse_length(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN + len + CRLF_LEN)
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for BulkString {
    fn from(s: &str) -> Self {
        BulkString(s.as_bytes().to_vec())
    }
}

impl From<String> for BulkString {
    fn from(value: String) -> Self {
        BulkString(value.into_bytes())
    }
}

impl From<&[u8]> for BulkString {
    fn from(s: &[u8]) -> Self {
        BulkString(s.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for BulkString {
    fn from(s: &[u8; N]) -> Self {
        BulkString(s.to_vec())
    }
}

impl AsRef<[u8]> for BulkString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::RespFrame;

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_bulk_string_encode() {
        let frame: RespFrame = BulkString::new("hello".as_bytes().to_vec()).into();
        assert_eq!(frame.encode(), b"$5\r\nhello\r\n");
    }

    #[test]
    fn test_null_bulk_string_encode() {
        let frame: RespFrame = BulkString::new([]).into();
        assert_eq!(frame.encode(), b"$-1\r\n");
    }

    #[test]
    fn test_bulk_string_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"$5\r\nhello\r\n");

        let frame = BulkString::decode(&mut buf)?;
        assert_eq!(frame, BulkString::new(b"hello"));

        buf.extend_from_slice(b"$5\r\nhello");
        let ret = BulkString::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        buf.extend_from_slice(b"\r\n");
        let frame = BulkString::decode(&mut buf)?;
        assert_eq!(frame, BulkString::new(b"hello"));

        Ok(())
    }

    #[test]
    fn test_null_bulk_string_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"$-1\r\n");

        let frame = BulkString::decode(&mut buf)?;
        assert_eq!(frame, BulkString::new([]));

        Ok(())
    }
}
