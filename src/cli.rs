//! Command line interface definition.

use crate::file_type::{file_type, FileType};
use clap::Parser;
use std::path::{Path, PathBuf};

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

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliApp {
    /// The target markdown or image file.
    #[arg(exclusive = true)]
    pub filepath: Option<PathBuf>,
    /// Set the token.
    #[arg(long, exclusive = true)]
    pub set_token: bool,
    /// Update the token.
    #[arg(long, exclusive = true)]
    pub update_token: bool,
    /// Delete the token.
    #[arg(long, exclusive = true)]
    pub delete_token: bool,
}
