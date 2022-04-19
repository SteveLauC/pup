//!result.rs: print manipulation results to the user

use colored::Colorize;
use std::error::Error;
use std::path::{PathBuf, Path};

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
    pub fn res_handling(&mut self, res: Vec<Result<String, Box<dyn Error>>>, image_paths: Vec<&Path>) {
        self.total += 1;
        assert_eq!(res.len(), image_paths.len());
        for (idx, image) in image_paths.iter().enumerate() {
            if let Err(msg) = &res[idx] {
                println!("find: {:?}\n[{}]: {:?}", image, "FAILED".red(), msg);
                self.failed += 1;
            } else {
                println!("find: {:?}\n[{}]", image, "DONE".green());
                self.done += 1;
            }
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
