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

fn main() {
    create_config();
    let config: Cfg = check_config();
    let cli_cfg: CliCfg = get_cli_config(cli_init());
    manipulate(cli_cfg, &config);
}
