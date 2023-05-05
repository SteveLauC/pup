///! Operations supported by `pup`.
///
/// See `Operation` for more details.
pub mod manipulation;
pub mod token;

use crate::{
    config::UserConfig,
    util::{
        adjust_pwd::adjust_pwd,
        file_type::{file_type, FileType},
        result::MdManipulationResult,
    },
    CliApp,
};
use anyhow::{anyhow, Result};
use manipulation::{img_manipulate, md_manipulate};
use std::{
    path::{Path, PathBuf},
    process::exit,
    sync::{Arc, Mutex},
};
use token::{delete_token, set_token, update_token};

/// Operation supported by `pup`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operation {
    /// Check the configuration file and the TOKEN.
    ///
    /// ```shell
    /// $ pup
    /// ```
    Check,
    /// TOKEN-related Operations
    ///
    /// ```shell
    /// $ pup --set-token
    /// $ pup --update-token
    /// $ pup --delete-token
    /// ```
    Token(TokenOperation),
    /// File manipulation
    ///
    /// ```shell
    /// $ pup xxx.md
    /// $ pup xxx.jpeg/jpg/png/gif
    /// ```
    File(TargetFile),
}

/// Token-related Operations
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenOperation {
    /// Set a TOKEN
    Set,
    /// Update the TOKEN.
    Update,
    /// Delete the TOKEN.
    Delete,
}

impl TokenOperation {
    /// Execute this `TokenOperation`
    pub fn execute(&self) -> Result<()> {
        match self {
            TokenOperation::Set => set_token()?,
            TokenOperation::Update => update_token()?,
            TokenOperation::Delete => delete_token()?,
        }

        Ok(())
    }
}

impl TryFrom<&CliApp> for Operation {
    type Error = anyhow::Error;

    /// Try to convert from `CliApp` to `Operation`
    ///
    /// # Error
    /// The ONLY error case that could occur is that the specified `filepath` does not
    /// exist.
    fn try_from(value: &CliApp) -> Result<Self, Self::Error> {
        if value.set_token {
            return Ok(Operation::Token(TokenOperation::Set));
        }

        if value.update_token {
            return Ok(Operation::Token(TokenOperation::Update));
        }

        if value.delete_token {
            return Ok(Operation::Token(TokenOperation::Delete));
        }

        if let Some(ref file_path) = value.filepath {
            if !file_path.exists() {
                return Err(anyhow!(format!(
                    "File {} does not exist.",
                    file_path.display()
                )));
            }

            let target_file = TargetFile::new(file_path);
            return Ok(Operation::File(target_file));
        }

        Ok(Operation::Check)
    }
}

impl Operation {
    /// Execute this `Operation`.
    pub fn execute(&self) -> Result<()> {
        match self {
            Operation::Check => {
                UserConfig::load();
            }
            Operation::Token(token_opt) => token_opt.execute()?,
            Operation::File(target_file) => {
                let user_config = UserConfig::load();
                match target_file.file_type {
                    FileType::Unknown => {
                        eprintln!("Unknown file type, abort.");
                        exit(1);
                    }
                    FileType::Markdown => {
                        let result = Arc::new(Mutex::new(MdManipulationResult::default()));
                        adjust_pwd(target_file.file_path.as_path());
                        md_manipulate(target_file, &user_config, result);
                    }
                    FileType::Image => img_manipulate(target_file, &user_config),
                };
            }
        }

        Ok(())
    }
}

/// Type to represent the file to be operated.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TargetFile {
    /// File path, the value of `[filename]` option
    pub file_path: PathBuf,
    /// File Type: Markdown or Image
    pub file_type: FileType,
}

impl TargetFile {
    pub fn new(file_path: &Path) -> Self {
        TargetFile {
            file_path: file_path.to_owned(),
            file_type: file_type(file_path),
        }
    }
}
