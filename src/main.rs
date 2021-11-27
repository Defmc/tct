use std::{env, process};
use std::error::Error;
use tct::cmd::COMMANDS;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    COMMANDS
        .get(args.get(1).unwrap_or_else(|| {
            eprintln!("need to pass an argument");
            process::exit(1);
        }).as_str())
        .unwrap_or_else(|| {
            eprintln!("{} isn't a valid command", args[1]);
            process::exit(1);
        })(&args[1..])
}
