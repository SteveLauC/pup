/*
 * manipulation.rs: Process the target markdown file, check line by line if there
 *                  is a markdown image link, send the request and replace the path
*/

use crate::cli::CliCfg;
use crate::config::Cfg;
use crate::encode::encode;
use crate::r#match::{is_matched, MatchedLine};
use crate::request::request;
use crate::response::get_url;
use rayon::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::ops::Index;
use std::path::Path;
// use anyhow::Result;
use crate::result::res_handling;
use reqwest::blocking::Response;
// use std::thread::sleep;
// use std::time::Duration;

/*
 * purpose: call the functions from other modules to complete the task
*/
pub fn manipulate(cli_cfg: CliCfg, config: &Cfg) {
    // buffer-wrapped target file
    let md_file: BufReader<File> =
        BufReader::new(File::open(cli_cfg.filename.as_path()).expect("can not open target file"));
    let mut lines: Vec<String> = md_file.lines().map(|item| item.unwrap()).collect();

    lines.par_iter_mut().for_each(|line| {
        line.push('\n');
        // Take a break to prevent the server from getting too busy and returning CONFLICT
        // sleep(Duration::from_millis(300));

        if is_matched(line) {
            let mut mth: MatchedLine = MatchedLine::new(line);
            // to escape simultaneous occurence of mutable and immutable borrowing
            let image_path = Path::new(mth.line.index(mth.range.clone())).to_path_buf();
            res_handling(
                manipulate_mthed_line(&mut mth, &cli_cfg, config),
                image_path.as_path(),
            )
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
    })
}

/*
 * purpose: deal with every matched line
*/
fn manipulate_mthed_line<'a>(
    mth: &'a mut MatchedLine<'a>,
    _cli_cfg: &CliCfg,
    config: &Cfg,
) -> Result<(), Box<dyn std::error::Error>> {
    let image_path: &Path = Path::new(mth.line.index(mth.range.clone()));
    let image_name = image_path.file_name().expect("can not get image name");

    let encoded_file_contents = encode(image_path)?;
    let res: Response = request(config, image_name.to_str().unwrap(), encoded_file_contents)?;
    let url: String = get_url(res)?;
    mth.replace(url.as_str());

    Ok(())
}
