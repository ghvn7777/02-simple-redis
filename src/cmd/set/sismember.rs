use crate::{
    cmd::{extract_args, validate_command, CommandError, CommandExecutor},
    RespArray, RespFrame,
};

#[derive(Debug, PartialEq, Eq)]
pub struct SIsmember {
    pub(crate) key: String,
    pub(crate) member: String,
}

impl CommandExecutor for SIsmember {
    fn execute(self, backend: &crate::Backend) -> RespFrame {
        match backend.sismember(&self.key, &self.member) {
            Some(value) => (value as i64).into(),
            None => RespFrame::Null(crate::RespNull),
        }
    }
}

impl TryFrom<RespArray> for SIsmember {
    type Error = CommandError;
    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["sismember"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(member))) => {
                Ok(SIsmember {
                    key: String::from_utf8(key.0)?,
                    member: String::from_utf8(member.0)?,
                })
            }
            _ => Err(CommandError::InvalidArgument(
                "Invalid key or field".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::RespDecode;
    use anyhow::Result;
    use bytes::BytesMut;

    use super::*;

    #[test]
    fn test_sismember_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$9\r\nsismember\r\n$5\r\nmyset\r\n$3\r\none\r\n");

        let frame = RespArray::decode(&mut buf)?;

        let result: SIsmember = frame.try_into()?;
        assert_eq!(result.key, "myset");
        assert_eq!(result.member, "one");

        Ok(())
    }
}
