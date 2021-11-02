use crate::cmd::ProgResult;
use std::fs::File;
use std::io::{stdout, StdoutLock, Write};

use memmap2::MmapOptions;

pub fn grep(items: &[String]) -> ProgResult {
    let out = stdout();
    let mut out = out.lock();

    let pattern = items[1].as_bytes();

    items
        .iter()
        .skip(2)
        .for_each(|filepath| file_grep(&mut out, pattern, filepath));
    Ok(())
}

#[inline(always)]
pub fn file_grep(out: &mut StdoutLock, pattern: &[u8], filepath: &str) {
    let buf_reader = unsafe {
        MmapOptions::new()
            .map(&File::open(filepath).expect("File not found"))
            .expect("Cant map file")
    };

    let (mut line_start, mut indx_seq) = (0, 0);
    buf_reader.iter().enumerate().for_each(|(i, &b)| {
        if indx_seq != pattern.len() {
            if b == pattern[indx_seq] {
                indx_seq += 1;
            } else {
                indx_seq = 0;
            }
        }

        if b == b'\n' {
            if indx_seq == pattern.len() {
                out.write_all(&buf_reader[line_start..i])
                    .expect("Cannot print");
                out.write_all(b"\n").unwrap();
                indx_seq = 0;
            }
            line_start = i + 1;
        }
    });
}
