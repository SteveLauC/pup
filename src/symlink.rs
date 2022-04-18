//! symlink.rs: parse softlink to get the real path

use std::path::{Path, PathBuf};

pub fn parse_symlink(link: &Path) -> PathBuf {
    let mut res = link.to_path_buf();
    while res.is_symlink() {
        res = res.read_link().expect("can not parse symlink");
    }
    res
}
