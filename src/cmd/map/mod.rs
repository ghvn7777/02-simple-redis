mod get;
mod set;

pub use get::*;
pub use set::*;

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{
        cmd::{CommandExecutor, RESP_OK},
        Backend, RespFrame,
    };

    use super::*;
    #[test]
    fn test_set_get_command() -> Result<()> {
        let backend = Backend::new();
        let cmd = Set {
            key: "hello".to_string(),
            value: RespFrame::BulkString(b"world".into()),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RESP_OK.clone());

        let cmd = Get {
            key: "hello".to_string(),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RespFrame::BulkString(b"world".into()));

        Ok(())
    }
}
