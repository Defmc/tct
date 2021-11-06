use crate::cmd::ProgResult;
use std::fs::copy;
use std::io::{stdout, Write};

pub fn cp(paths: &[String]) -> ProgResult {
    let out = stdout();
    let mut out = out.lock();
    let mut iter = paths.iter().skip(1).peekable();

    while iter.peek().is_some() {
        if let Err(error) = copy(
            iter.next().unwrap(),
            iter.next().expect("Missing target folder"),
        ) {
            out.write_all(error.to_string().as_bytes())
                .expect("Cannot print");
        }
    }
    Ok(())
}
