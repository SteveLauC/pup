//! handles everything relevant to command line interface
//!     
//! This command line tool can be used in the following ways;
//!     1. `pup`: check the configuration file and TOKEN, then exit.
//!     2. `pup markdown`: upload the pics used in filename
//!     3. `pup image`: upload image and store the returned URL to clipboard
//!     4. `pup --update-token`: set or update the TOKEN
//!     5. `pup --delete-token`: delete the TOKEN

use crate::{
    file_type::{file_type, FileType},
    token::{delete_token, update_token},
};
use clap::{crate_authors, crate_description, crate_version, Arg, ArgMatches, Command};
use std::{
    path::{Path, PathBuf},
    process::exit,
};

/// type to represent the command line interface configuration
pub struct CliCfg {
    pub file_path: PathBuf,
    pub file_type: FileType,
}

impl CliCfg {
    fn new(file_name: &Path) -> Self {
        CliCfg {
            file_path: file_name.to_owned(),
            file_type: file_type(file_name),
        }
    }
}

/// purpose: initialize the command line application
pub fn cli_init() -> ArgMatches {
    Command::new("pup")
        .author(crate_authors!(" "))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("filename")
                .takes_value(true)
                .exclusive(true)
                .help("The target markdown or image file"),
        )
        .arg(
            Arg::new("delete token")
                .takes_value(false)
                .required(false)
                .exclusive(true)
                .help("delete the current TOKEN")
                .long("delete-token"),
        )
        .arg(
            Arg::new("update token")
                .required(false)
                .takes_value(false)
                .exclusive(true)
                .help("update the TOKEN")
                .long("update-token"),
        )
        .get_matches()
}

/// purpose: handle two TOKEN-relevant special cli options
///
/// action: call `update_token()` or `delete_token()` if the corresponding
///         option is present, then exit the program
pub fn token_opt(app: &ArgMatches) {
    if app.is_present("update token") {
        if update_token().is_err() {
            eprintln!("Failed to update the TOKEN");
            exit(1);
        } else {
            exit(0);
        }
    }

    if app.is_present("delete token") {
        if delete_token().is_err() {
            eprintln!("Failed to delete the TOKEN");
            exit(1);
        } else {
            exit(0);
        }
    }
}

/// purpose: initialize a cli config struct if pup is executed like `pup target-file`
///
/// action:
///     `$ pup --delete-token` or `$ pup --update-token`:
///         call `delete_token()` or `update_token()` if the corresponding option
///         is given, then exit the program.
///      `$ pup filename`:
///         if filename exists, return Some(CliCfg), otherwise warn the user and exit the program
///      `pup`:
///         return None
/// arguments:
///     * `app`: return value of `cli_init`
///
/// return: optional CliCfg
///
/// note: we need to make sure `filename` exists
pub fn get_cli_config(app: ArgMatches) -> Option<CliCfg> {
    // Handle two token-related opt
    token_opt(&app);

    if app.is_present("filename") {
        let file = Path::new(app.value_of("filename").unwrap());
        if file.exists() {
            Some(CliCfg::new(file))
        } else {
            eprintln!("pup: {:?} does not exist", file);
            exit(1);
        }
    } else {
        None
    }
}
