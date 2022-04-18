//! encode.rs: encode file contents

use base64::{encode_config_slice, STANDARD};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;


/// Error type of encoding process
#[derive(Debug)]
pub enum EncodeFailedCases {
    CanNotOpenFile,
    CanNotReadFile,
}
// impl Debug + Display for our own error type so that 
// we can have std::error::Error implmented
impl Display for EncodeFailedCases {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for EncodeFailedCases {}



/// purpose: to encode the image contents
/// 
/// arguments:
///     * `path`: image path
///     
/// return: If successful, return the encoded bytes. 
///         Otherwise return the corressponding error type.
pub fn encode(path: &Path) -> Result<Vec<u8>, EncodeFailedCases> {
    // buffer for the original file contents
    let mut orig_contents: Vec<u8> = Vec::new();
    if let Ok(mut image_file) = File::open(path) {
        if image_file.read_to_end(&mut orig_contents).is_ok() {
            // buffer for the encoded contents
            let mut encoded_contents: Vec<u8> = Vec::new();
            encoded_contents.resize(orig_contents.len() * 4 / 3 + 4, 0);
            let n_bytes: usize =
                encode_config_slice(orig_contents, STANDARD, &mut encoded_contents);
            encoded_contents.truncate(n_bytes); // remove the trailing zeros

            Ok(encoded_contents)
        } else {
            Err(EncodeFailedCases::CanNotReadFile)
        }
    } else {
        Err(EncodeFailedCases::CanNotOpenFile)
    }
}
