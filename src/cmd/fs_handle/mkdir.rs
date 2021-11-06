use crate::cmd::ProgResult;
use std::fs;

pub fn mkdir(paths: &[String]) -> ProgResult {
    paths
        .iter()
        .skip(1)
        .for_each(|path| fs::create_dir_all(path).unwrap());
    Ok(())
}
