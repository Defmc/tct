use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

pub mod cat;
pub mod doc;
pub mod fs_handle;
pub mod grep;

type ProgResult = Result<(), Box<dyn std::error::Error>>;
type Prog = fn(&[String]) -> ProgResult;

macro_rules! impl_man {
    ($hmp:tt, $($cmd:tt), *) => {
        $(
            $hmp.insert($cmd, include_str!(concat!("../../man/", $cmd))).unwrap();
        )*
    }
}

lazy_static! {
    pub static ref COMMANDS: FxHashMap<&'static str, Prog> = {
        let mut hmp: FxHashMap<&'static str, Prog> = FxHashMap::default();
        hmp.insert("cat", cat::cat);
        hmp.insert("man", doc::man);
        hmp.insert("grep", grep::grep);
        hmp.insert("rm", fs_handle::rm);
        hmp.insert("can", fs_handle::can);
        hmp.insert("touch", fs_handle::touch);
        hmp.insert("rmdir", fs_handle::rmdir);
        hmp.insert("mkdir", fs_handle::mkdir);
        hmp.insert("check", doc::check);
        hmp
    };
    pub static ref MANUALS: FxHashMap<&'static str, &'static str> = {
        let mut hmp: FxHashMap<&'static str, &'static str> = FxHashMap::default();
        impl_man!(hmp, "cat", "man");
        hmp
    };
}
