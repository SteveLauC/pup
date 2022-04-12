//! everything relevant to configuration.

use std::io::Write;
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;


/// Person type, used by `author` and `commiter` arguments
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub email: String,
    pub date: String,
}

/// arguments needed to send the http request
#[derive(Debug)]
pub struct Config {
    pub accept: String,
    pub owner: String,
    pub path: String,
    pub message: String,
    pub content: String,
    pub branch: String,
    pub commiter: Person,
    pub author: Person,
    pub token: String,
}

// configuration file template
const TEMPLETE: &str = r#"
# configuration file for pup
[user]

[request-configuration]
accept = ""
owner = ""
path = ""
message = ""
branch = ""
token = ""
"#;


/// create configure file `config.toml` under `~/.config/pup/` if it doesn't
/// exist, and write configuration template to it.
pub fn create_config() {
    let config_file: PathBuf = PathBuf::from("~/.config/pup/config.toml");

    if !config_file.exists() {
        let mut f: File = OpenOptions::new()
            .create_new(true)
            .mode(0o600)
            .open(config_file).expect("can not create configure file");
        f.write_all(TEMPLETE.as_bytes()).expect("can not write configuration template to ~/.config/pup/config.toml");
    }
}

/// check the configure file to see whether it is valid
/// if not, print the erroneous config to stderr
pub fn check_config() {

}
