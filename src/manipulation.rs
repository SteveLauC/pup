//! Process target file.
//!
//! For a Markdown file, check line by line if there is a markdown
//! image link, send the request and replace the path
//!
//! For a image, upload it the GitHub repo and place the returned URL into the
//! system clipboard.

use crate::{
    cli::TargetFile, config::UserConfig, r#match::MatchedLine,
    request::Uploader, response::get_url, result::MdManipulationResult,
};
use arboard::Clipboard;
use colored::Colorize;
use rayon::prelude::*;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    ops::Index,
    path::Path,
    sync::{Arc, Mutex},
};

/// Call the functions from other modules to complete the task
pub fn md_manipulate(
    target_file: &TargetFile,
    config: &UserConfig,
    res: Arc<Mutex<MdManipulationResult>>,
) {
    // buffer-wrapped target file
    let md_file = BufReader::new(
        File::open(target_file.file_path.as_path())
            .expect("can not open target file"),
    );
    let mut lines = md_file
        .lines()
        .map(|item| item.unwrap())
        .collect::<Vec<String>>();
    let uploader = Uploader::init(config.token.as_str());

    lines.par_iter_mut().for_each(|line| {
        line.push('\n');

        if let Some(mut mth) = MatchedLine::new(line) {
            // to escape simultaneous occurrence of mutable and immutable borrowing
            let image_path =
                Path::new(mth.line.index(mth.range.clone())).to_path_buf();
            res.lock().unwrap().res_handling(
                manipulate_mthed_line(&uploader, &mut mth, config),
                image_path.as_path(),
            );
        }
    });

    // write to the target markdown file
    let mut md_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(target_file.file_path.as_path())
        .expect("can not open target markdown file for writing purpose");

    lines.iter().for_each(|line| {
        md_file
            .write(line.as_bytes())
            .expect("can not write to the target markdown file");
    });

    // print the statistical result
    println!("{}", res.lock().unwrap());
}

/// Deal with every matched line.
///
/// This is the helper function used inside of [`md_manipulate`].
#[inline]
fn manipulate_mthed_line<'a>(
    uploader: &Uploader,
    mth: &'a mut MatchedLine<'a>,
    config: &UserConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let image_path = mth.line.index(mth.range.clone());
    let response = uploader.upload(image_path, config)?;
    let url = get_url(response)?;
    mth.replace(url.as_str());
    Ok(())
}

/// Places `contents` into the clipboard.
///
/// A helper function used in [`img_manipulate`].
#[inline]
fn clipboard_set(contents: &str) {
    let mut clipboard =
        Clipboard::new().expect("failed to initialize a clipboard instance");
    clipboard
        .set_text(contents)
        .expect("can not store the returned URL to clipboard");
}

/// Manipulate single image file
pub fn img_manipulate(target_file: &TargetFile, config: &UserConfig) {
    let uploader = Uploader::init(config.token.as_str());
    let response = uploader
        .upload(target_file.file_path.as_path(), config)
        .expect("Failed to upload");

    let url = get_url(response)
        .expect("failed to extract URL from the response message");
    clipboard_set(url.as_str());

    println!("{} [{}]", target_file.file_path.display(), "done".green());
    println!("URL: {} (stored to your clipboard)", url.as_str().purple());
}
