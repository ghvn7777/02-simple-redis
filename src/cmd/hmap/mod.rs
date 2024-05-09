mod hget;
mod hgetall;
mod hmget;
mod hset;

pub use hget::*;
pub use hgetall::*;
pub use hmget::*;
pub use hset::*;

#[cfg(test)]
mod tests {

    use crate::{cmd::CommandExecutor, Backend, BulkString, RespArray, RespFrame, RespNull};
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
        assert_eq!(result, 1.into());

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

        let cmd = HSet {
            key: "myhash".to_string(),
            field: "field1".to_string(),
            value: RespFrame::BulkString(b"Hello".into()),
        };
        cmd.execute(&backend);

        let cmd = HSet {
            key: "myhash".to_string(),
            field: "field2".to_string(),
            value: RespFrame::BulkString(b"World".into()),
        };
        cmd.execute(&backend);

        let cmd = HMGet {
            gets: vec![
                HGet {
                    key: "myhash".to_string(),
                    field: "field1".to_string(),
                },
                HGet {
                    key: "myhash".to_string(),
                    field: "field2".to_string(),
                },
                HGet {
                    key: "myhash".to_string(),
                    field: "nofield".to_string(),
                },
            ],
        };
        let ret = cmd.execute(&backend);
        let expected = RespArray::new([
            BulkString::from("Hello").into(),
            BulkString::from("World").into(),
            RespNull.into(),
        ]);
        assert_eq!(ret, expected.into());

        Ok(())
    }
}
