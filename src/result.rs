//!result.rs: print manipulation results to the user

use colored::Colorize;
use std::error::Error;
use std::path::Path;

#[derive(Default)]
pub struct Res {
    total: usize,
    done: usize,
    failed: usize,
}

impl Res {
    /// purpose: handle the result of function `manipulate_mthed_line`
    ///
    /// arguments:
    ///     * `res`: return value of `manipulate_mthed_line`
    ///     * `image`: image path
    pub fn res_handling(&mut self, res: Result<(), Box<dyn Error>>, image_path: &Path) {
        self.total += 1;
        if let Err(msg) = res {
            println!("find: {:?}\n[{}]: {:?}", image_path, "FAILED".red(), msg);
            self.failed += 1;
        } else {
            println!("find: {:?}\n[{}]", image_path, "DONE".green());
            self.done += 1;
        }
    }

    /// purpose: print the statictical result to the user
    pub fn show_results(&self) {
        println!(
            "\npup: {} {}, {} {}(✓), {} {}(✗)",
            self.total,
            "FOUND".bold(),
            self.done,
            "SUCCESSFUL".bold().green(),
            self.failed,
            "FAILED".bold().red()
        );
    }
}
