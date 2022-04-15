/*
 * config.rs: Handle everything relevant to `$HOME/.config/pup/config.toml`, including:
 *             1. Initialize config file
 *             2. Check that every field of the configuration file is not empty
 *             3. Instantiate a valid `Cfg` struct
*/

use colored::Colorize;
use std::env::var;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::process::exit;
use toml::Value;

// configuration file template
const TEMPLETE: &str = r#"# configuration file for pup
[user]
github-user-name = ""
github-repo-name = ""
mail = ""

[authorization]
token = ""
"#;

// constant relative path of config file and folder
const RELATIVE_CONFIG_FOLDER: &str = ".config/pup";
const RELATIVE_CONFIG_FILE: &str = ".config/pup/config.toml";

#[derive(Debug)]
pub struct Cfg {
    pub name: String,
    pub repo: String,
    pub mail: String,
    pub token: String,
}

// return HOME path
fn home_path() -> String {
    if let Ok(home) = var("HOME") {
        home
    } else {
        eprintln!("{} variable is unset.", "HOME".bold().red());
        exit(1);
    }
}

/*
 * purpose: return absolute config folder path
*/
fn config_folder_path() -> PathBuf {
    PathBuf::from(format!("{}/{}", home_path(), RELATIVE_CONFIG_FOLDER))
}

/*
 * purpose: return absolute config file path
*/
fn config_file_path() -> PathBuf {
    PathBuf::from(format!("{}/{}", home_path(), RELATIVE_CONFIG_FILE))
}

/*
    purpose: initialize configuration file
    action: if the config already exists, do nothing.
            Otherwise, create and write TEMPLATE to it.
*/
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

/*
 * purpose: check every fields of the config file to see if any of them is empty.
 *          If so, warn user and exit the program.
 * action:  read the file contents and parse it.
 * return:  A initialized `Cfg` struct.
*/
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
                    let token: &str = config["authorization"]["token"]
                        .as_str()
                        .expect("config.toml: missing authorization/token field");

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
                    if token.is_empty() {
                        eprintln!("{} is unset.", "token".red());
                        field_empty_sign = true;
                    }
                    if field_empty_sign {
                        exit(1);
                    }

                    Cfg {
                        name: name.into(),
                        repo: repo.into(),
                        mail: mail.into(),
                        token: format!("token {}", token),
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

[authorization]
token = "secert_token"
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
        assert_eq!(
            val["authorization"]["token"].as_str().unwrap(),
            "secert_token"
        );
    }
}
