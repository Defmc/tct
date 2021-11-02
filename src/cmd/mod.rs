use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

pub mod cat;
pub mod grep;
pub mod man;

type ProgResult = Result<(), Box<dyn std::error::Error>>;
type Prog = fn(&[String]) -> ProgResult;

lazy_static! {
    pub static ref COMMANDS: FxHashMap<&'static str, Prog> = {
        let mut hmp: FxHashMap<&'static str, Prog> = FxHashMap::default();
        hmp.insert("cat", cat::cat);
        hmp.insert("man", man::man);
        hmp.insert("grep", grep::grep);
        hmp
    };
    pub static ref MANUALS: FxHashMap<&'static str, &'static str> = {
        let mut hmp: FxHashMap<&'static str, &'static str> = FxHashMap::default();
        hmp.insert("cat", include_str!("../../man/cat"));
        hmp.insert("man", include_str!("../../man/man"));
        hmp
    };
}
