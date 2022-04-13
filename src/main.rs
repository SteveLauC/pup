mod cli;
use cli::{cli_init, get_target_file};

mod config;
use config::{check_config, create_config, Config};

mod request;

use std::path::PathBuf;

fn main() {
    let file_name: PathBuf = get_target_file(cli_init());
    println!("{:?}", file_name);

    create_config();
    let config: Option<Config> = check_config();
    println!("{:?}", config);
}
