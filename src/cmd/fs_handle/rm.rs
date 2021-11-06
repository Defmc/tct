use crate::cmd::ProgResult;
use std::fs;

pub fn rm(paths: &[String]) -> ProgResult {
    paths
        .iter()
        .skip(1)
        .for_each(|path| fs::remove_file(path).unwrap());
    Ok(())
}
