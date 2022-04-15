mod cli;
mod config;
mod manipulation;
mod r#match;
mod request;
mod response;

use anyhow::Result;
use cli::{cli_init, get_cli_config, CliCfg};
use config::{check_config, create_config, Cfg};
use manipulation::manipulate;
use std::path::PathBuf;

fn main() -> Result<()> {
    // cli initialization
    let cli_cfg: CliCfg = get_cli_config(cli_init());

    // create_config();
    // let config: Cfg = check_config();
    // let target: PathBuf = get_target_file(cli_init());
    // manipulate(target.as_path(), &config)?;
    Ok(())
}
