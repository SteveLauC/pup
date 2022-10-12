#![warn(missing_debug_implementations)]

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
    cli::{cli_init, get_cli_config},
    config::{check_config, create_config, Cfg},
    file_type::FileType,
    manipulation::md_manipulate,
    result::MdManipulationResult,
};
use std::{
    env::set_current_dir,
    fs::canonicalize,
    process::exit,
    sync::{Arc, Mutex},
};
use crate::manipulation::img_manipulate;

fn main() {
    create_config();

    // if filename option is given
    if let Some(cli_cfg) = get_cli_config(cli_init()) {
        let config: Cfg = check_config();
        // change current working directory to the parent dir of the markdown doc
        // so we can handling relative path
        let mut md_file = canonicalize(cli_cfg.file_path.as_path())
            .expect("Failed to get absolute path of target markdown file");
        md_file.pop();
        set_current_dir(md_file.as_path())
            .expect("Failed to set current dir to the parent of the markdown doc");

        let res: Arc<Mutex<MdManipulationResult>> = Arc::new(Mutex::new(MdManipulationResult::default()));

        match cli_cfg.file_type {
            FileType::Unknown => {
                eprintln!("Unknown file type, abort.");
                exit(1);
            }
            FileType::Markdown => md_manipulate(&cli_cfg, &config, res),
            FileType::Image => img_manipulate(&cli_cfg, &config),
        };
    }

    // when no argument is given, all we need to do is to check the configuration file and TOKEN.
    check_config();
}
