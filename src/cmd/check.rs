use crate::cmd::{ProgResult, COMMANDS};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
struct CmdCheckError(String);

impl Display for CmdCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} isn't a valid command", self.0)
    }
}

impl Error for CmdCheckError {}

pub fn check(cmds: &[String]) -> ProgResult {
    for cmd in cmds.iter().skip(1) {
        if !COMMANDS.contains_key(cmd.as_str()) {
            return Err(Box::new(CmdCheckError(cmd.clone())));
        }
    }
    Ok(())
}
