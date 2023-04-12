//! Encode file contents

use anyhow::Result;
use base64::{encode_engine_slice, engine::DEFAULT_ENGINE};
use std::{fs::read, path::Path};

/// Encode the image contents
pub fn encode(path: &Path) -> Result<Vec<u8>> {
    let orig_contents = read(path)?;
    // buffer for the encoded contents
    let mut encoded_contents = Vec::new();
    encoded_contents.resize(orig_contents.len() * 4 / 3 + 4, 0);
    let n_bytes = encode_engine_slice(orig_contents, &mut encoded_contents, &DEFAULT_ENGINE);
    // remove the trailing zeros
    encoded_contents.truncate(n_bytes);

    Ok(encoded_contents)
}
