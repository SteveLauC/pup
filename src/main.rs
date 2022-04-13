mod cli;
use cli::{cli_init, get_target_file};

mod config;
use config::{check_config, create_config, Config};

mod request;
use request::request;

mod r#match;
use r#match::replace;

use anyhow::Result;
use std::path::PathBuf;


fn main() -> Result<()> {
    let mut local_path: String = String::from("> ![title](/home/steve/doc.png)xx");
    let target_url = "https://github.com/SteveLauC/pic/blob/main/Screen%20Shot%202022-04-06%20at%2010.30.49%20AM.png";
    replace(&mut local_path, target_url);
    println!("{}", local_path);
    Ok(())        
}
