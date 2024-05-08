use enum_dispatch::enum_dispatch;

use crate::{RespArray, RespFrame};

use super::{CommandError, Echo, Get, HGet, HGetAll, HMGet, HSet, Set, Unrecognized};

#[enum_dispatch(CommandExecutor)]
#[derive(Debug)]
pub enum Command {
    Get(Get),
    Set(Set),
    HGet(HGet),
    HSet(HSet),
    HGetAll(HGetAll),
    HMGet(HMGet),

    Echo(Echo),

    // unrecognized command
    Unrecognized(Unrecognized),
}

impl TryFrom<RespFrame> for Command {
    type Error = CommandError;

    fn try_from(v: RespFrame) -> Result<Self, Self::Error> {
        match v {
            RespFrame::Array(value) => value.try_into(),
            _ => Err(CommandError::InvalidCommand(
                "Command must be an Array".to_string(),
            )),
        }
    }
}

impl TryFrom<RespArray> for Command {
    type Error = CommandError;

    fn try_from(v: RespArray) -> Result<Self, Self::Error> {
        match v.first() {
            Some(RespFrame::BulkString(ref cmd)) => match cmd.as_ref() {
                b"GET" | b"get" => Ok(Command::Get(Get::try_from(v)?)),
                b"SET" | b"set" => Ok(Command::Set(Set::try_from(v)?)),
                b"HGET" | b"hget" => Ok(Command::HGet(HGet::try_from(v)?)),
                b"HSET" | b"hset" => Ok(Command::HSet(HSet::try_from(v)?)),
                b"HGETALL" | b"hgetall" => Ok(Command::HGetAll(HGetAll::try_from(v)?)),
                b"HMGET" | b"hmget" => Ok(Command::HMGet(HMGet::try_from(v)?)),
                b"ECHO" | b"echo" => Ok(Command::Echo(Echo::try_from(v)?)),
                _ => Ok(Unrecognized.into()),
            },
            _ => Err(CommandError::InvalidCommand(
                "Command must have a BulkString as the first argument".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use bytes::BytesMut;

    use crate::{cmd::CommandExecutor, Backend, RespDecode, RespNull};

    use super::*;

    #[test]
    fn test_command() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nget\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf)?;
        let cmd: Command = frame.try_into()?;

        let backend = Backend::new();

        let ret = cmd.execute(&backend);
        assert_eq!(ret, RespFrame::Null(RespNull));

        Ok(())
    }
}
