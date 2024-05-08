mod hget;
mod hgetall;
mod hset;

pub use hget::*;
pub use hgetall::*;
pub use hset::*;

#[cfg(test)]
mod tests {

    use crate::{
        cmd::{CommandExecutor, RESP_OK},
        Backend, BulkString, RespArray, RespFrame,
    };
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_hset_hget_hgetall_commands() -> Result<()> {
        let backend = Backend::new();
        let cmd = HSet {
            key: "map".to_string(),
            field: "hello".to_string(),
            value: RespFrame::BulkString(b"world".into()),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RESP_OK.clone());

        let cmd = HSet {
            key: "map".to_string(),
            field: "hello1".to_string(),
            value: RespFrame::BulkString(b"world1".into()),
        };
        cmd.execute(&backend);

        let cmd = HGet {
            key: "map".to_string(),
            field: "hello".to_string(),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RespFrame::BulkString(b"world".into()));

        let cmd = HGetAll {
            key: "map".to_string(),
            sort: true,
        };
        let result = cmd.execute(&backend);
        let expected = RespArray::new([
            BulkString::from("hello").into(),
            BulkString::from("world").into(),
            BulkString::from("hello1").into(),
            BulkString::from("world1").into(),
        ]);

        assert_eq!(result, expected.into());

        Ok(())
    }
}
