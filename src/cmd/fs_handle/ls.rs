use crate::cmd::ProgResult;
use std::fs::{read_dir, DirEntry};
use std::io::{stdout, StdoutLock, Write};

pub fn ls(paths: &[String]) -> ProgResult {
    let out = stdout();
    let mut out = out.lock();

    paths.iter().skip(1).for_each(|path| {
        read_dir(path)
            .unwrap()
            .for_each(|entry| show_file(&mut out, entry.expect("Directory not found")))
    });
    out.write_all(b"\n").expect("Cannot print");
    Ok(())
}

#[inline(always)]
pub fn show_file(out: &mut StdoutLock, dir: DirEntry) {
    out.write_all(dir.path().file_name().unwrap().to_string_lossy().as_bytes())
        .expect("Cannot print");
    out.write_all(b" ").unwrap();
}
