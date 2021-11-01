use std::env;
use std::error::Error;
use tct::cmd;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let res = cmd::COMMANDS
        .get(args[1].as_str())
        .unwrap_or_else(|| panic!("{} isn't a valid command", args[1].as_str()))(
        &args[1..]
    );
    res
}
