//! everything relevant to configuration.
use std::env::var;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{PathBuf};
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

const RELATIVE_CONFIG_FOLDER: &str = ".config/pup";
const RELATIVE_CONFIG_FILE: &str = ".config/pup/config.toml";

#[derive(Debug)]
pub struct Config {
    name: String,
    repo: String,
    mail: String,
    token: String,
}

/// return HOME path
fn home_path() -> String {
    var("HOME").expect("HOME variable is not defined")
}

/// return abslote config foler path
fn config_folder_path() -> PathBuf {
    let mut config_folder_path: PathBuf = PathBuf::new();
    config_folder_path.push(home_path());
    config_folder_path.push(RELATIVE_CONFIG_FOLDER);

    config_folder_path
}

/// return abslote config file path 
fn config_file_path() -> PathBuf {
    let mut config_file_path: PathBuf = PathBuf::new();
    config_file_path.push(home_path());
    config_file_path.push(RELATIVE_CONFIG_FILE);

    config_file_path
}

/// create configuration folder and file when they don't exist
/// write configuration template to the cofig file when the file was just created
pub fn create_config() {
    let config_folder_path: PathBuf = config_folder_path();
    let config_file_path: PathBuf = config_file_path();
    if !config_folder_path.exists() {
        create_dir(config_folder_path).expect("can not create configuration folder");
    }

    if !config_file_path.exists() {
        let mut f: File = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(config_file_path)
            .expect("can not create configuration file");
        f.write_all(TEMPLETE.as_bytes())
            .expect("can not write configuration template to ~/.config/pup/config.toml");
    }
}

/// check the configure file to see whether it is valid
/// if not, print the erroneous config to stderr
pub fn check_config() -> Option<Config> {
    let config_path: PathBuf = config_file_path();

    let mut f: File = File::open(config_path).expect("can not open configuration file");
    let mut buf: String = String::with_capacity(200);
    f.read_to_string(&mut buf)
        .expect("can not read configuration file");

    // parse the configuration file
    let config: Value = buf
        .parse::<Value>()
        .expect("the configuration file is invalid");

    let name: &str = config["user"]["github-user-name"].as_str().unwrap();
    let repo: &str = config["user"]["github-repo-name"].as_str().unwrap();
    let mail: &str = config["user"]["mail"].as_str().unwrap();
    let token: &str = config["authorization"]["token"].as_str().unwrap();
    let mut config_unset: bool = false;

    if name.is_empty() {
        eprintln!("name = \"\", name is unset.");
        config_unset = true;
    }
    if repo.is_empty() {
        eprintln!("repo = \"\", repo is unset.");
        config_unset = true;
    }
    if mail.is_empty() {
        eprintln!("mail = \"\", mail is unset.");
        config_unset = true;
    }
    if token.is_empty() {
        eprintln!("token = \"\", token is unset.");
        config_unset = true;
    }
    if config_unset {
        exit(1);
    }

    Some(Config {
        name: name.into(),
        repo: repo.into(),
        mail: mail.into(),
        token: token.into(),
    })
}

mod test {
    use super::*;

    #[test]
    fn template_parse() {
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
