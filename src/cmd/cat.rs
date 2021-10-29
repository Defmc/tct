use crate::cmd::ProgResult;
use std::fs::File;
use std::io::{stdout, Write, BufReader, prelude::*};

pub fn cat(paths: &[String]) -> ProgResult {
    let stdout = stdout();
    let mut lock = stdout.lock();
    let mut buf = String::with_capacity(1024);

    paths.iter().skip(1).for_each(|path| {
        let mut buf_reader = BufReader::new(File::open(path).expect("Cannot open file"));
        buf_reader.read_to_string(&mut buf).expect("Cannot write in internal program buffer");
        write!(
            lock,
            "{}",
            buf
        )
        .expect("Cannot write in stdout")
    });
    lock.flush().expect("Cannot flush stdout");
    Ok(())
}
