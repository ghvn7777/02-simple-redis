use crate::{
    cmd::{extract_args, validate_command, CommandError, CommandExecutor},
    RespArray, RespFrame,
};

use super::HGet;

#[derive(Debug)]
pub struct HMGet {
    pub(crate) gets: Vec<HGet>,
}

impl CommandExecutor for HMGet {
    fn execute(self, backend: &crate::Backend) -> RespFrame {
        let mut array = RespArray::new([]);

        for get in self.gets {
            array.push(get.execute(backend));
        }

        array.into()
    }
}

impl TryFrom<RespArray> for HMGet {
    type Error = CommandError;
    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hmget"], usize::MAX)?;

        let mut args = extract_args(value, 1)?.into_iter();
        let key = if let Some(RespFrame::BulkString(key)) = args.next() {
            String::from_utf8_lossy(&key.0).to_string()
        } else {
            return Err(CommandError::InvalidArgument(
                "Invalid arguments".to_string(),
            ));
        };

        let mut gets = Vec::new();
        for arg in args {
            if let RespFrame::BulkString(field) = arg {
                let field = String::from_utf8_lossy(&field.0).to_string();
                gets.push(HGet {
                    key: key.clone(),
                    field,
                });
            } else {
                return Err(CommandError::InvalidArgument(
                    "Invalid arguments".to_string(),
                ));
            }
        }

        Ok(HMGet { gets })
    }
}

#[cfg(test)]
mod tests {

    use crate::RespDecode;
    use anyhow::Result;
    use bytes::BytesMut;

    use super::*;

    #[test]
    fn test_hmget_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*4\r\n$5\r\nhmget\r\n$3\r\nmap\r\n$5\r\nhello\r\n$5\r\nworld\r\n");

        let frame = RespArray::decode(&mut buf)?;

        let result: HMGet = frame.try_into()?;
        let expected = vec![
            HGet {
                key: "map".to_string(),
                field: "hello".to_string(),
            },
            HGet {
                key: "map".to_string(),
                field: "world".to_string(),
            },
        ];
        assert_eq!(result.gets, expected);

        Ok(())
    }
}
