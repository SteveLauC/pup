/*
 * result.rs: print manipulation results to the user
*/

use std::fmt::{Debug, Display};
use std::path::Path;

/*
 * purpose: To reduce the number of calls to println
*/
pub fn line_manipulation_print<T, Y>(path: T, res: Y, msg: &str)
where
    T: AsRef<Path> + Debug,
    Y: Display,
{
    println!("find: {:?}\n{}: {}", path, res, msg);
}
