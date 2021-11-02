use crate::cmd::ProgResult;
use std::fs::File;
use std::io::{stdout, StdoutLock, Write};
use std::ops::ControlFlow;

use memmap2::MmapOptions;

pub fn grep(items: &[String]) -> ProgResult {
    let out = stdout();
    let mut out = out.lock();

    let pattern = items[1].as_bytes();

    items.iter().skip(2).for_each(|filepath| {
        file_grep(&mut out, pattern, filepath);
    });
    Ok(())
}

#[inline(always)]
pub fn file_grep(out: &mut StdoutLock, pattern: &[u8], filepath: &str) {
    let buf_reader = unsafe {
        MmapOptions::new()
            .map(&File::open(filepath).expect("File not fond"))
            .expect("Cant map file")
    };
    buf_reader.split(|&x| x == b'\n').for_each(|line| {
        if contains_slice(line, pattern) {
            out.write_all(line).expect("Cannot print");
            out.write_all(b"\n").unwrap();
        }
    })
}

pub fn contains_slice(lhs: &[u8], rhs: &[u8]) -> bool {
    match lhs.iter().try_fold(0usize, |indx_seq, &c| {
        if c == rhs[indx_seq] {
            if indx_seq == rhs.len() - 1 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(indx_seq + 1)
            }
        } else {
            ControlFlow::Continue(0)
        }
    }) {
        ControlFlow::Break(_) => true,
        ControlFlow::Continue(_) => false,
    }
}
