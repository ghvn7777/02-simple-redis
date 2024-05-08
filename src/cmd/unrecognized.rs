use crate::{Backend, RespFrame};

use super::{CommandExecutor, RESP_OK};

#[derive(Debug)]
pub struct Unrecognized;

impl CommandExecutor for Unrecognized {
    fn execute(self, _: &Backend) -> RespFrame {
        RESP_OK.clone()
    }
}
