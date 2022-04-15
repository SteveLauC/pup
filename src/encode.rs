/*
 * encode.rs: encode file contents
*/

use anyhow::Result;
use base64::{encode_config_slice, STANDARD};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/*
 * purpose: to encode the file
*/
pub fn encode(path: &Path) -> Result<Vec<u8>> {
    // buffer for the original file contents
    let mut orig_contents: Vec<u8> = Vec::new();
    let mut image_file: File = File::open(path)?;
    image_file.read_to_end(&mut orig_contents)?;

    // buffer for the encoded contents
    let mut encoded_contents: Vec<u8> = Vec::new();
    encoded_contents.resize(orig_contents.len() * 4 / 3 + 4, 0);
    let n_bytes: usize = encode_config_slice(orig_contents, STANDARD, &mut encoded_contents);
    encoded_contents.truncate(n_bytes); // remove the trailing zeros

    Ok(encoded_contents)
}
