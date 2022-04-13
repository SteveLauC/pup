mod cli;
use cli::{cli_init, get_target_file};

mod config;
use config::{check_config, create_config, Config};

mod request;
use request::request;

use anyhow::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    Ok(())
}
