use crate::cmd::ProgResult;
use std::fs;

pub fn rmdir(paths: &[String]) -> ProgResult {
    paths
        .iter()
        .skip(1)
        .for_each(|path| fs::remove_dir_all(path).unwrap());
    Ok(())
}
