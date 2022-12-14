//! A command-line tool that automatically uploads images from the markdown
//! document to the GitHub repo and replaces the paths with the returned URL.

#![cfg(unix)]
#![crate_name = "pup"]
#![deny(unused)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(missing_docs)]

mod cli;
mod config;
mod echo;
mod encode;
mod file_type;
mod manipulation;
mod r#match;
mod request;
mod response;
mod result;
mod token;

use crate::{
    cli::{cli_init, get_target_file},
    config::{get_user_config, init_config},
    file_type::FileType,
    manipulation::img_manipulate,
    manipulation::md_manipulate,
    result::MdManipulationResult,
};
use std::{
    env::set_current_dir,
    fs::canonicalize,
    path::Path,
    process::exit,
    sync::{Arc, Mutex},
};

// Change current working directory to the parent directory of the markdown doc
// so we can handle relative path.
#[inline]
fn adjust_pwd(target_markdown_file_path: &Path) {
    let md_absolute_path = canonicalize(target_markdown_file_path)
        .expect("Failed to get absolute path of target markdown file");
    let md_file_parent_dir = md_absolute_path
        .parent()
        .expect("The target Markdown doc should have a parent directory");
    set_current_dir(md_file_parent_dir)
        .expect("Failed to set current dir to the parent of the markdown doc");
}

fn main() {
    init_config();
    let user_config = get_user_config();

    // if filename option is given
    if let Some(target_file) = get_target_file(&cli_init()) {
        match target_file.file_type {
            FileType::Unknown => {
                eprintln!("Unknown file type, abort.");
                exit(1);
            }
            FileType::Markdown => {
                let result =
                    Arc::new(Mutex::new(MdManipulationResult::default()));
                adjust_pwd(target_file.file_path.as_path());
                md_manipulate(&target_file, &user_config, result);
            }
            FileType::Image => img_manipulate(&target_file, &user_config),
        };
    }
}
