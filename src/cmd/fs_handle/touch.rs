use crate::cmd::ProgResult;
use std::fs;

pub fn touch(paths: &[String]) -> ProgResult {
    paths.iter().for_each(|path| fs::write(path, b"").unwrap());
    Ok(())
}
