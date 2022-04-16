/*
 * result.rs: print manipulation results to the user
*/

use colored::{ColoredString, Colorize};
// use std::fmt::Display;
use std::error::Error;
use std::path::Path;
// use anyhow::Result;
/*
 * purpose: To reduce the number of calls to println
*/
pub fn line_manipulation_print(path: &Path, res: ColoredString) {
    // match msg {
    //     Some(msg) => println!("find: {:?}\n{}: {}", path, res, msg),
    //     None => println!("find: {:?}\n{}", path, res),
    // }
    println!("find: {:?}\n{}", path, res);
}

pub fn res_handling(res: Result<(), Box<dyn Error>>, image_path: &Path) {
    if let Err(msg) = res {
        println!("{}", msg);
        line_manipulation_print(image_path, "Failed".red());
    } else {
        line_manipulation_print(image_path, "DONE".green());
    }
}
