use crate::cmd::ProgResult;
use std::fs::File;
use std::io::{stdout, Write};

use memmap2::MmapOptions;

pub fn cat(paths: &[String]) -> ProgResult {
    let out = stdout();
    let mut out = out.lock();

    paths.iter().skip(1).for_each(|path| {
        let buf_reader = unsafe {
            MmapOptions::new()
                .map(&File::open(path).expect("File not found"))
                .unwrap()
        };
        out.write_all(&buf_reader[..])
            .expect("Cannot show file content");
    });
    out.flush()?;
    Ok(())
}
