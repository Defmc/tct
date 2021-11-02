use std::env;
use std::error::Error;
use tct::cmd::COMMANDS;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    COMMANDS
        .get(args[1].as_str())
        .unwrap_or_else(|| panic!("{} isn't a valid command", args[1]))(&args[1..])
}
