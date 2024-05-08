mod command;
mod echo;
mod hmap;
mod map;
mod set;
mod unrecognized;

use crate::{Backend, RespArray, RespError, RespFrame, SimpleString};
use anyhow::Result;
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use thiserror::Error;

pub use {
    command::Command,
    echo::Echo,
    hmap::{HGet, HGetAll, HMGet, HSet},
    map::{Get, Set},
    set::{SAdd, SIsmember},
    unrecognized::Unrecognized,
};

// you could also use once_cell instead of lazy_static
lazy_static! {
    static ref RESP_OK: RespFrame = SimpleString::new("OK".to_string()).into();
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("{0}")]
    RespError(#[from] RespError),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

#[enum_dispatch]
pub trait CommandExecutor {
    fn execute(self, backend: &Backend) -> RespFrame;
}

pub fn validate_command(
    value: &RespArray,
    names: &[&'static str],
    n_args: usize,
) -> Result<(), CommandError> {
    if n_args != usize::MAX && value.len() != n_args + names.len() {
        return Err(CommandError::InvalidArgument(format!(
            "{} command must have exactly {} argument",
            names.join(" "),
            n_args
        )));
    }

    for (i, name) in names.iter().enumerate() {
        match value[i] {
            RespFrame::BulkString(ref cmd) => {
                if cmd.as_ref().to_ascii_lowercase() != name.as_bytes() {
                    return Err(CommandError::InvalidCommand(format!(
                        "Invalid command: expected {}, got {}",
                        name,
                        String::from_utf8_lossy(cmd)
                    )));
                }
            }
            _ => {
                return Err(CommandError::InvalidCommand(
                    "Command must have a BulkString as the first argument".to_string(),
                ));
            }
        }
    }

    Ok(())
}

fn extract_args(value: RespArray, start: usize) -> Result<Vec<RespFrame>, CommandError> {
    Ok(value.0.into_iter().skip(start).collect::<Vec<RespFrame>>())
}
