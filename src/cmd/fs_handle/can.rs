use crate::cmd::ProgResult;
use std::fs;
use std::io::{stdout, Write};

pub fn can(paths: &[String]) -> ProgResult {
    let out = stdout();
    let mut out = out.lock();

    paths.iter().for_each(|path| {
        out.write_all(fs::canonicalize(path).unwrap().to_str().unwrap().as_bytes())
            .expect("Cannot print");
        out.write_all(b"\n").unwrap()
    });
    Ok(())
}
