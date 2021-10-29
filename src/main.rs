use std::env;
use std::error::Error;
use std::time::Instant;
pub use tct::cmd;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let start = Instant::now();
    let res = cmd::COMMANDS
        .get(args[1].as_str())
        .expect(format!("{} isn't a valid command", args[1]).as_str())(&args[1..]);
    println!("\nElapsed time: {:?}", start.elapsed());
    res
}
