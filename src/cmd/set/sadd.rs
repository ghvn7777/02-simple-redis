use crate::{
    cmd::{extract_args, validate_command, CommandError, CommandExecutor},
    RespArray, RespFrame,
};

#[derive(Debug)]
pub struct SAdd {
    pub(crate) key: String,
    pub(crate) member: String,
}

impl CommandExecutor for SAdd {
    fn execute(self, backend: &crate::Backend) -> RespFrame {
        (backend.sadd(self.key, self.member) as i64).into()
    }
}

impl TryFrom<RespArray> for SAdd {
    type Error = CommandError;
    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["sadd"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(member))) => Ok(SAdd {
                key: String::from_utf8(key.0)?,
                member: String::from_utf8(member.0)?,
            }),
            _ => Err(CommandError::InvalidArgument(
                "Invalid key or value".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use bytes::BytesMut;

    use crate::RespDecode;

    use super::*;

    #[test]
    fn test_sadd_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$4\r\nsadd\r\n$5\r\nmyset\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf)?;

        let result: SAdd = frame.try_into()?;
        assert_eq!(result.key, "myset");
        assert_eq!(result.member, "hello");

        Ok(())
    }
}
