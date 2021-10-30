use crate::cmd::ProgResult;
use std::io::{stdout, Write};

pub fn grep(items: &[String]) -> ProgResult {
    let out = stdout();
    let mut out = out.lock();

    items.skip(2).for_each(|file| {

    })
}
