//!result.rs: print manipulation results to the user

use colored::{ColoredString, Colorize};
use std::error::Error;
use std::path::Path;

///  purpose: To reduce the number of calls to println
///  arguments: 
///      * `path`: iamge path
///      * `res`: green "DONE" or red "FAILED" to indicate the manipulation results
///      * `msg`: if res is "FAILED", msg is Some(error message), else it is None
pub fn line_manipulation_print(path: &Path, res: ColoredString, msg: Option<Box<dyn Error>>) {
    match msg {
        Some(msg) => println!("find: {:?}\n{}: {}", path, res, msg),
        None => println!("find: {:?}\n{}", path, res),
    }
}

/// purpose: handle the result of function `manipulate_mthed_line`
/// arguments: 
///     * `res`: return value of `manipulate_mthed_line`
///     * `image`: image path
pub fn res_handling(res: Result<(), Box<dyn Error>>, image_path: &Path) {
    if let Err(msg) = res {
        line_manipulation_print(image_path, "FAILED".red(), Some(msg));
    } else {
        line_manipulation_print(image_path, "DONE".green(), None);
    }
}
