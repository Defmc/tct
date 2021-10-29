use crate::cmd::ProgResult;
use std::fs;
use std::io::{stdout, Write};

pub fn cat(paths: &[String]) -> ProgResult {
    let out = stdout();
    let mut out = out.lock();

    write!(
        out,
        "{}",
        paths
            .iter()
            .skip(1)
            .map(|path| fs::read_to_string(path).expect("Cannot extract file content"))
            .collect::<String>()
    )?;

    out.flush()?;
    Ok(())
}
