//! config.rs: Handles everything relevant to configuration file, including:
//!             1. Initialize config file
//!             2. Check that every field of the configuration file is not empty
//!             3. Instantiate a valid `Cfg` struct
//!
//! configuration file location: if you have $XDG_CONFIG_HOME validly set, then
//! it is located in $XDG_CONFIG_HOME/pup; Otherwise, it is under `$HOME/.config/pup`

use crate::token::fetch_token;
use colored::Colorize;
use std::env::var;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::process::exit;
use toml::Value;

/// configuration file template
const TEMPLETE: &str = r#"# configuration file for pup
[user]
github-user-name = ""
github-repo-name = ""
mail = ""
"#;

/// constant relative path of config file and folder
const RELATIVE_CONFIG_FOLDER: &str = "pup";

/// type to represent the user configuration
#[derive(Debug)]
pub struct Cfg {
    pub name: String,
    pub repo: String,
    pub mail: String,
    pub token: String,
}

/// purpose: return $XDG_CONFIG_HOME
///
/// action: read $XDG_CONFIG_HOME, if it is set and not empty, return it
///
/// return: if $XDG_CONFIG_HOME is set and not empty, return Some($XDG_CONFIG_HOME)
///         otherwise, return None
fn xdg_config_home() -> Option<String> {
    if let Ok(config_home) = var("XDG_CONFIG_HOME") {
        if config_home.is_empty() {
            return Some(config_home);
        } else {
            return None;
        }
    }
    None
}

/// purpose: return home directory
///
/// action: read $HOME, if it is set, return it
///
/// return: if $HOME is set and not empty, return $HOME
///         otherwise, warn user and exit the program
fn home_path() -> String {
    if let Ok(home) = var("HOME") {
        home
    } else {
        eprintln!("{} environment variable is unset.", "HOME".bold().red());
        exit(1);
    }
}

/// purpose: return absolute config folder path
///
/// action: if $XDG_CONFIG_HONE is set and not empty, return `$XDG_CONFIG_HOME/pup`
///         otherwise, return `$HOME/.config/pup`
///
/// return: `$XDG_CONFIG_HOME/pup` or `$HOME/.config/pup`
fn config_folder_path() -> PathBuf {
    if let Some(xdg_config) = xdg_config_home() {
        PathBuf::from(format!("{}/{}", xdg_config, RELATIVE_CONFIG_FOLDER))
    } else {
        PathBuf::from(format!(
            "{}/.config/{}",
            home_path(),
            RELATIVE_CONFIG_FOLDER
        ))
    }
}

/// purpose: return absolute config file path
///
/// action: concatenate `config_folder_path()` and `config.toml`, then return
///
/// return: `config_folder_path()` + `config.toml`
fn config_file_path() -> PathBuf {
    let mut config_folder: PathBuf = config_folder_path();
    config_folder.push("config.toml");
    config_folder
}

/// purpose: initialize configuration file
///
/// action: if the config already exists, do nothing.
///        Otherwise, create and write TEMPLATE to it.
pub fn create_config() {
    let config_folder_path: PathBuf = config_folder_path();
    let config_file_path: PathBuf = config_file_path();

    if !config_folder_path.exists() {
        if let Err(msg) = create_dir(config_folder_path.as_path()) {
            eprintln!(
                "pup: can not create {:?} due to {}",
                config_folder_path, msg
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
                eprintln!("pup: can not create {:?} due to {}", config_file_path, msg);
                exit(1);
            }
        }
    }
}

/// purpose: check every fields of the config file to see if any of them is empty.
///          If so, warn user and exit the program.
///
/// action:  read the file contents and parse it.
///
/// return:  A initialized `Cfg` struct.
pub fn check_config() -> Cfg {
    let config_path: PathBuf = config_file_path();

    match File::open(config_path.as_path()) {
        Ok(mut file) => {
            // buf to store the config file contents
            let mut buf: String = String::with_capacity(200);
            file.read_to_string(&mut buf)
                .expect("pup: can not read config file contents");

            // parse the configuration file
            match buf.parse::<Value>() {
                Ok(config) => {
                    let name: &str = config["user"]["github-user-name"]
                        .as_str()
                        .expect("config.toml: missing user/github-user-name field");
                    let repo: &str = config["user"]["github-repo-name"]
                        .as_str()
                        .expect("config.toml: missing user/github-repo-name field");
                    let mail: &str = config["user"]["mail"]
                        .as_str()
                        .expect("config.toml: missing user/mail field");

                    // sign used to record whether there are empty fields
                    let mut field_empty_sign: bool = false;

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
                    Cfg {
                        name: name.into(),
                        repo: repo.into(),
                        mail: mail.into(),
                        token: format!("token {}", fetch_token()),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn template_parse_test() {
        let tem: &str = r#"# configuration file for pup
[user]
github-user-name = "SteveLauC"
github-repo-name = "pic"
mail = "stevelauc@outlook.com"
"#;
        let val: Value = tem.parse::<Value>().unwrap();
        assert_eq!(
            val["user"]["github-user-name"].as_str().unwrap(),
            "SteveLauC"
        );
        assert_eq!(val["user"]["github-repo-name"].as_str().unwrap(), "pic");
        assert_eq!(
            val["user"]["mail"].as_str().unwrap(),
            "stevelauc@outlook.com"
        );
    }

    #[test]
    fn config_folder_file_relation_path() {
        let config_folder_path: PathBuf = config_folder_path();
        let mut config_file_path: PathBuf = config_file_path();
        config_file_path.pop();

        assert_eq!(config_folder_path, config_file_path);
    }

    #[test]
    fn config_folder_test() {
        if let Ok(xdg_config) = var("XDG_CONFIG_HOME") {
            if !xdg_config.is_empty() {
                assert_eq!(
                    PathBuf::from(format!("{}/pup", xdg_config)),
                    config_folder_path()
                );
            } else {
                assert_eq!(
                    PathBuf::from(format!("{}/.config/pup", home_path())),
                    config_folder_path()
                );
            }
        }
    }
}
