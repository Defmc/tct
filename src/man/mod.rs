use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

type ManualMap = FxHashMap<&'static str, &'static str>;

macro_rules! impl_man {
    ($hmp:tt, $($cmd:tt), *) => {
        $(
            $hmp.insert($cmd, $cmd).unwrap();
        )*
    }
}

lazy_static! {
    pub static ref MANUALS: ManualMap = {
        let mut hmp: ManualMap = ManualMap::default();
        impl_man!(
            hmp,
            "cat",
            "man",
            "grei",
            "fs_handle/can",
            "fs_handle/cp",
            "fs_handle/cpdir",
            "fs_handle/ls",
            "fs_handle/mkdir",
            "fs_handle/rm",
            "fs_handle/rmdir",
            "fs_handle/touch"
        );
        hmp
    };
}
