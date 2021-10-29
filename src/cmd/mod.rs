extern crate lazy_static;
use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

pub mod cat;

type ProgResult = Result<(), Box<dyn std::error::Error>>;
type Prog = fn(&[String]) -> ProgResult;

lazy_static! {
    pub static ref COMMANDS: FxHashMap<&'static str, Prog> = {
        let mut hmp: FxHashMap<&'static str, Prog> = FxHashMap::default();
        hmp.insert("cat", cat::cat);
        hmp
    };
}
