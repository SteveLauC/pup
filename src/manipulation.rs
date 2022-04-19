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
use std::collections::HashMap;
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
            let image_paths = mth.paths.clone();
            r.lock().unwrap().res_handling(
                manipulate_mthed_line(&mut mth, config),
                image_paths.iter().map(|item| item.as_path()).collect()
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

// return Vec<Result<(), Error>>
// Ok() indicates success
// Error return failure cause
fn manipulate_mthed_line<'a>(
    mth: &'a mut MatchedLine<'a>,
    config: &Cfg,
) -> Vec<Result<String, Box<dyn std::error::Error>>> {

    let image_path_vec: Vec<PathBuf> = mth.range.iter().map(|range|{
        Path::new(mth.line.index(range.clone())).to_path_buf()
    }).collect();
    
    let res: Vec<Result<String, Box<dyn std::error::Error>>> = image_path_vec.iter().enumerate().map(|(_, image_path)|{
        manipulate_path(image_path, config)     
    }).collect();
    
    let urls: HashMap<usize, &str> = res.iter().enumerate().filter(|(_, item)|{
        item.is_ok()
    }).map(|(idx, res)|{
        (idx, res.as_ref().unwrap().as_str())
    }).collect();

    mth.replace(urls);

    res
}

fn manipulate_path(path: &Path, config: &Cfg) -> Result<String, Box<dyn std::error::Error>> {
    let image_name = path.file_name().expect("can not get image name");
    let encoded_file_contents = encode(path)?;
    let res: Response = request(config, image_name.to_str().unwrap(), encoded_file_contents)?;
    let url: String = get_url(res)?;
    Ok(url)
}