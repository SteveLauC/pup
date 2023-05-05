//! A command-line tool that automatically uploads images from the markdown
//! document to the GitHub repo and replaces the paths with the returned URL.

#![cfg(unix)]
#![crate_name = "pup"]
#![deny(unused)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(missing_docs)]

mod cli;
mod config;
mod echo;
mod encode;
mod file_type;
mod manipulation;
mod r#match;
mod operation;
mod request;
mod response;
mod result;
mod token;
mod util;

use crate::{cli::CliApp, config::init_config, operation::Operation};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    init_config();
    let app = CliApp::parse();
    let op = Operation::try_from(&app)?;
    op.execute()?;

    Ok(())
}
