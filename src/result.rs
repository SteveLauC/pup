//! Manipulation result handling

use colored::Colorize;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    path::Path,
};

/// Markdown file manipulation statistical result
#[derive(Default, Debug)]
pub struct MdManipulationResult {
    total: usize,
    done: usize,
    failed: usize,
}

impl MdManipulationResult {
    /// Check out the result of our image manipulation and report it to the user.
    pub fn res_handling(
        &mut self,
        res: Result<(), Box<dyn Error>>,
        image_path: &Path,
    ) {
        self.total += 1;
        if let Err(msg) = res {
            println!("find: {:?}\n[{}]: {:?}", image_path, "FAILED".red(), msg);
            self.failed += 1;
        } else {
            println!("find: {:?}\n[{}]", image_path, "DONE".green());
            self.done += 1;
        }
    }
}

/// Make [MdManipulationResult] printable so that we can display the statistics
/// when task finishes.
impl Display for MdManipulationResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "\npup: {} {}, {} {}(✓), {} {}(✗)",
            self.total,
            "FOUND".bold(),
            self.done,
            "SUCCESSFUL".bold().green(),
            self.failed,
            "FAILED".bold().red()
        )
    }
}
