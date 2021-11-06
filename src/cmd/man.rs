use crate::cmd::ProgResult;
use crate::man::MANUALS;
use std::io::{stdout, Write};

pub fn man(cmds: &[String]) -> ProgResult {
    let out = stdout();
    let mut out = out.lock();

    cmds.iter().skip(1).for_each(|cmd| {
        if !MANUALS.contains_key(cmd.as_str()) {
            out.write_all(MANUALS[cmd.as_str()].as_bytes())
                .expect("Cannot show manual");
        }
    });
    Ok(())
}
