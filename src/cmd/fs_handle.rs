use crate::cmd::ProgResult;
use std::fs;
use std::io::{stdout, Write};

pub fn mkdir(paths: &[String]) -> ProgResult {
    paths
        .iter()
        .for_each(|path| fs::create_dir_all(path).unwrap());
    Ok(())
}

pub fn rm(paths: &[String]) -> ProgResult {
    paths.iter().for_each(|path| fs::remove_file(path).unwrap());
    Ok(())
}

pub fn rmdir(paths: &[String]) -> ProgResult {
    paths
        .iter()
        .for_each(|path| fs::remove_dir_all(path).unwrap());
    Ok(())
}

pub fn touch(paths: &[String]) -> ProgResult {
    paths.iter().for_each(|path| fs::write(path, b"").unwrap());
    Ok(())
}

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
