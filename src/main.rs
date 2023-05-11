//! A command-line tool that automatically uploads images from the markdown
//! document to the GitHub repo and replaces the paths with the returned URL.

#![cfg(unix)]
#![crate_name = "pup"]
#![deny(unused)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(missing_docs)]

mod config;
mod operation;
mod util;

use crate::{config::init_config, operation::Operation};
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Command line interface.
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

fn main() -> Result<()> {
    init_config();
    let app = CliApp::parse();
    let op = Operation::try_from(&app)?;
    op.execute()?;

    Ok(())
}
