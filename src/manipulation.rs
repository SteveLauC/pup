//! manipulation.rs: Process the target markdown file, check line by line if there
//!                 is a markdown image link, send the request and replace the path

use crate::cli::CliCfg;
use crate::config::Cfg;
use crate::encode::encode;
use crate::r#match::MatchedLine;
use crate::request::request;
use crate::response::get_url;
use crate::result::Res;
use rayon::prelude::*;
use reqwest::blocking::Response;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::ops::Index;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// purpose: call the functions from other modules to complete the task
/// arguments:
///     * `cli_cfg`: command-line interface configuration
///     * `config`: user configuration
pub fn manipulate(cli_cfg: CliCfg, config: &Cfg, r: Arc<Mutex<Res>>) {
    // buffer-wrapped target file
    let md_file: BufReader<File> =
        BufReader::new(File::open(cli_cfg.filename.as_path()).expect("can not open target file"));
    let mut lines: Vec<String> = md_file.lines().map(|item| item.unwrap()).collect();

    lines.par_iter_mut().for_each(|line| {
        line.push('\n');

        if let Some(mut mth) = MatchedLine::new(line) {
            // to escape simultaneous occurence of mutable and immutable borrowing
            let image_path = Path::new(mth.line.index(mth.range.clone())).to_path_buf();
            r.lock().unwrap().res_handling(
                manipulate_mthed_line(&mut mth, config),
                image_path.as_path(),
            );
        }
    });

    // write to the target markdown file
    let mut md_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(cli_cfg.filename.as_path())
        .expect("can not open target markdown file for writing purpose");

    lines.iter().for_each(|line| {
        md_file
            .write(line.as_bytes())
            .expect("can not write to the target markdown file");
    });

    r.lock().unwrap().show_results();
}

/// purpose: deal with every matched line
///
/// arguments:
///     * `mth`: matched line
///     * `config`: user configuration
fn manipulate_mthed_line<'a>(
    mth: &'a mut MatchedLine<'a>,
    config: &Cfg,
) -> Result<(), Box<dyn std::error::Error>> {
    let image_path: PathBuf = Path::new(mth.line.index(mth.range.clone())).to_path_buf();

    let image_name = image_path.file_name().expect("can not get image name");

    let encoded_file_contents = encode(image_path.as_path())?;
    let res: Response = request(config, image_name.to_str().unwrap(), encoded_file_contents)?;
    let url: String = get_url(res)?;
    mth.replace(url.as_str());

    Ok(())
}
