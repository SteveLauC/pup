mod cli;
mod config;
mod encode;
mod manipulation;
mod r#match;
mod request;
mod response;
mod result;
mod symlink;

use cli::{cli_init, get_cli_config, CliCfg};
use config::{check_config, create_config, Cfg};
use manipulation::manipulate;
use result::Res;
use std::sync::{Arc, Mutex};

fn main() {
    create_config();
    let config: Cfg = check_config();
    let cli_cfg: CliCfg = get_cli_config(cli_init());

    let res: Arc<Mutex<Res>> = Arc::new(Mutex::new(Res::default()));
    manipulate(cli_cfg, &config, res);
}
