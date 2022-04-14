#![allow(dead_code)]
#![allow(unused_imports)]

mod cli;
use cli::{cli_init, get_target_file};

mod config;
use config::{check_config, create_config, Config};

mod request;
use request::request;

mod r#match;
use r#match::MatchedLine;

use anyhow::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    Ok(())
}
