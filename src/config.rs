//! Handles everything relevant to the User Configuration, including:
//!
//! 1. Initialize a config file
//! 2. Check that every field of the configuration file is not empty
//! 3. Instantiate a valid `UserConfig` struct
//!
//! Configuration file location:
//!
//! |Platform | Value                                                                 |
//! | ------- | ----------------------------------------------------------------------|
//! | Linux   | `$XDG_CONFIG_HOME/pup/config.toml` or `$HOME`/.config/pup/config.toml |
//! | macOS   | `$HOME`/Library/Application Support/pup/config.toml                   |

use crate::token::fetch_token;
use dirs::config_dir;
use serde::Deserialize;
use std::fs::read_to_string;
use std::{
    fs::{create_dir, OpenOptions},
    io::Write,
    os::unix::fs::OpenOptionsExt,
    path::PathBuf,
    process::exit,
};

/// User configuration file template
const TEMPLETE: &str = r#"# Configuration file for pup
github_user_name = "your_user_name"
github_repo_name = "your_repo_name"
mail = "your_mail_address"
"#;

/// type to represent the user configuration
#[derive(Debug, Deserialize)]
pub struct UserConfig {
    pub github_user_name: String,
    pub github_repo_name: String,
    pub mail: String,
    // This field is `Option`al since it will NOT be present in the
    // configuration file, to make `serde` successfully parse it without any
    // error, will make it `Option`al here.
    //
    // But an initialized `UserConfig` will NEVER have this field set to `None`,
    // feel free to call `.unwrap()` on it.
    pub token: Option<String>,
}

impl UserConfig {
    /// Try to construct an `UserConfig`.
   pub fn load() -> Self {
       let config_path = config_file_path();
       let config_file_contents = read_to_string(config_path.as_path())
           .expect("pup: can not read config file");

       match toml::from_str::<UserConfig>(&config_file_contents) {
           Ok(mut config) => {
               config.token = Some(fetch_token());
               config
           }
           Err(msg) => {
               eprintln!(
                   "pup: can not parse the configuration file due to: `{}`",
                   msg
               );
               exit(1);
           }
       }
   }
}

/// Return config directory path
///
/// |Platform | Value                                                                 |
/// | ------- | ----------------------------------------------------------------------|
/// | Linux   | `$XDG_CONFIG_HOME/pup` or `$HOME`/.config/pup                         |
/// | macOS   | `$HOME`/Library/Application Support/pup                               |
fn config_dir_path() -> PathBuf {
    let mut path = config_dir().unwrap();
    path.push("pup");
    path
}

/// Return config file path
///
/// |Platform | Value                                                                 |
/// | ------- | ----------------------------------------------------------------------|
/// | Linux   | `$XDG_CONFIG_HOME/pup/config.toml` or `$HOME`/.config/pup/config.toml |
/// | macOS   | `$HOME`/Library/Application Support/pup/config.toml                   |
fn config_file_path() -> PathBuf {
    let mut path = config_dir().unwrap();
    path.push("pup");
    path.push("config.toml");
    path
}

/// Initialize configuration file
///
/// If the config already exists, do nothing. Otherwise, create and write
/// TEMPLATE to it.
pub fn init_config() {
    let config_dir_path = config_dir_path();
    let config_file_path = config_file_path();

    if !config_dir_path.exists() {
        if let Err(msg) = create_dir(config_dir_path.as_path()) {
            eprintln!(
                "pup: can not create {:?} due to {}",
                config_dir_path, msg
            );
            exit(1);
        }
    }

    if !config_file_path.exists() {
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(config_file_path.as_path())
        {
            Ok(mut file) => {
                if let Err(msg) = file.write_all(TEMPLETE.as_bytes()) {
                    eprintln!(
                        "pup: can not write to {:?} due to {}",
                        config_file_path, msg
                    );
                    exit(1);
                }
            }
            Err(msg) => {
                eprintln!(
                    "pup: can not create {:?} due to {}",
                    config_file_path, msg
                );
                exit(1);
            }
        }
    }
}
