use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

pub mod cat;
pub mod fs_handle;
pub mod grep;
pub mod man;

pub use fs_handle::*;

type ProgResult = Result<(), Box<dyn std::error::Error>>;
type Prog = fn(&[String]) -> ProgResult;

macro_rules! impl_man {
    ($hmp:tt, $($cmd:tt), *) => {
        $(
            $hmp.insert($cmd, include_str!(concat!("../../man/", $cmd))).unwrap();
        )*
    }
}

macro_rules! impl_cmd {
    ($hmp:tt, $($cmd:tt), *) => {
        $(
            $hmp.insert(stringify!($cmd), $cmd::$cmd);
        )*
    }
}

lazy_static! {
    pub static ref COMMANDS: FxHashMap<&'static str, Prog> = {
        let mut hmp: FxHashMap<&'static str, Prog> = FxHashMap::default();
        impl_cmd!(hmp, cat, man, grep, rm, can, touch, rmdir, mkdir);
        hmp
    };
    pub static ref MANUALS: FxHashMap<&'static str, &'static str> = {
        let mut hmp: FxHashMap<&'static str, &'static str> = FxHashMap::default();
        impl_man!(hmp, "cat", "man");
        hmp
    };
}
