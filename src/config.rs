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
use colored::Colorize;
use dirs::config_dir;
use std::{
    fs::{create_dir, File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::OpenOptionsExt,
    path::PathBuf,
    process::exit,
};
use toml::Value;

/// User configuration file template
const TEMPLETE: &str = r#"# configuration file for pup
[user]
github-user-name = ""
github-repo-name = ""
mail = ""
"#;

/// type to represent the user configuration
#[derive(Debug)]
pub struct UserConfig {
    pub name: String,
    pub repo: String,
    pub mail: String,
    pub token: String,
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

/// Try to construct an `UserConfig`.
///
/// This function will check every fields of the config file to see if any of
/// them is empty. If so, warn user and exit the program.
pub fn get_user_config() -> UserConfig {
    let config_path = config_file_path();

    match File::open(config_path.as_path()) {
        Ok(mut file) => {
            // buf to store the config file contents
            let mut buf: String = String::with_capacity(200);
            file.read_to_string(&mut buf)
                .expect("pup: can not read config file contents");

            // parse the configuration file
            match buf.parse::<Value>() {
                Ok(config) => {
                    let name =
                        config["user"]["github-user-name"].as_str().expect(
                            "config.toml: missing user/github-user-name field",
                        );
                    let repo =
                        config["user"]["github-repo-name"].as_str().expect(
                            "config.toml: missing user/github-repo-name field",
                        );
                    let mail = config["user"]["mail"]
                        .as_str()
                        .expect("config.toml: missing user/mail field");

                    // sign used to record whether there are empty fields
                    let mut field_empty_sign = false;

                    if name.is_empty() {
                        eprintln!("{} is unset.", "name".red());
                        field_empty_sign = true;
                    }
                    if repo.is_empty() {
                        eprintln!("{} is unset.", "repo".red());
                        field_empty_sign = true;
                    }
                    if mail.is_empty() {
                        eprintln!("{} is unset.", "mail".red());
                        field_empty_sign = true;
                    }
                    if field_empty_sign {
                        exit(1);
                    }
                    UserConfig {
                        name: name.into(),
                        repo: repo.into(),
                        mail: mail.into(),
                        token: fetch_token(),
                    }
                }
                Err(msg) => {
                    eprintln!("pup: invalid config file due to {}", msg);
                    exit(1);
                }
            }
        }
        Err(msg) => {
            eprintln!("pup: can not read {:?} due to {}", config_path, msg);
            exit(1);
        }
    }
}
