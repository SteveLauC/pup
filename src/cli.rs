//! Handles everything relevant to command line interface
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
use clap::{
    crate_authors, crate_description, crate_version, Arg, ArgAction,
    ArgMatches, Command,
};
use std::{
    path::{Path, PathBuf},
    process::exit,
};

/// Type to represent the file to be operated.
pub struct TargetFile {
    /// File path, the value of `[filename]` option
    pub file_path: PathBuf,
    /// File Type: Markdown or Image
    pub file_type: FileType,
}

impl TargetFile {
    fn new(file_path: &Path) -> Self {
        TargetFile {
            file_path: file_path.to_owned(),
            file_type: file_type(file_path),
        }
    }
}

/// Initialize the command line application
pub fn cli_init() -> ArgMatches {
    Command::new("pup")
        .author(crate_authors!(" "))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("filepath")
                .action(ArgAction::Set)
                .exclusive(true)
                .help("The target markdown or image file"),
        )
        .arg(
            Arg::new("delete token")
                .action(ArgAction::SetTrue)
                .required(false)
                .exclusive(true)
                .help("delete the current TOKEN")
                .long("delete-token"),
        )
        .arg(
            Arg::new("update token")
                .required(false)
                .action(ArgAction::SetTrue)
                .exclusive(true)
                .help("update the TOKEN")
                .long("update-token"),
        )
        .get_matches()
}

/// handle two TOKEN-relevant special cli options
///
/// call `update_token()` or `delete_token()` if the corresponding option is
/// present, then exit the program
pub fn token_opt(app: &ArgMatches) {
    if app.get_flag("update token") {
        if update_token().is_err() {
            eprintln!("Failed to update the TOKEN");
            exit(1);
        } else {
            exit(0);
        }
    }

    if app.get_flag("delete token") {
        if delete_token().is_err() {
            eprintln!("Failed to delete the TOKEN");
            exit(1);
        } else {
            exit(0);
        }
    }
}

/// Initialize a cli config struct if pup is executed like `pup target-file`
///
/// #### Action
///
/// * `$ pup --delete-token` or `$ pup --update-token`:
///
///   call `delete_token()` or `update_token()` if the corresponding option
///   is given, then exit the program.
/// * `$ pup filename`:
///
///   if filepath exists, return Some(CliCfg), otherwise warn the user
///   and exit the program
/// * `pup`:
///
///   return None
///
/// NOTE: we need to make sure `filepath` exists
pub fn get_target_file(app: &ArgMatches) -> Option<TargetFile> {
    // Handle two token-related opt
    token_opt(app);

    if let Some(path) = app.get_one::<String>("filepath") {
        let path = Path::new(path);
        if path.exists() {
            Some(TargetFile::new(path))
        } else {
            eprintln!("pup: {:?} does not exist", path);
            exit(1);
        }
    } else {
        None
    }
}
