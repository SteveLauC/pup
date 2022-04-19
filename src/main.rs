mod cli;
mod config;
mod encode;
mod manipulation;
mod r#match;
mod request;
mod response;
mod result;

use cli::{cli_init, get_cli_config, CliCfg};
use config::{check_config, create_config, Cfg};
use manipulation::manipulate;
use result::Res;
use std::env::set_current_dir;
use std::fs::canonicalize;
use std::sync::{Arc, Mutex};

fn main() {
    create_config();
    let config: Cfg = check_config();
    let cli_cfg: CliCfg = get_cli_config(cli_init());

    // change current working directory to the parent dir of the markdown doc
    // so we can handling relative path
    let mut md_file = canonicalize(cli_cfg.filename.as_path())
        .expect("Failed to get absolute path of target markdown file");
    md_file.pop();
    set_current_dir(md_file.as_path())
        .expect("Failed to set current dir to the parent of the markdown doc");

    let res: Arc<Mutex<Res>> = Arc::new(Mutex::new(Res::default()));
    manipulate(cli_cfg, &config, res);
}
