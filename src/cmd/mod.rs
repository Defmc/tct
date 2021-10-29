extern crate lazy_static;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub mod cat;

type ProgResult = Result<(), Box<dyn std::error::Error>>;
type Prog = fn(&[String]) -> ProgResult;

lazy_static! {
    pub static ref COMMANDS: HashMap<&'static str, Prog> = {
        let mut hmp: HashMap<&'static str, Prog> = HashMap::new();
        hmp.insert("cat", cat::cat);
        hmp
    };
}
