/*
 * manipulation.rs: Process the target markdown file, check line by line if there
 *                  is a markdown image link, send the request and replace the path
*/

use super::cli::CliCfg;
use super::config::Cfg;
use super::encode::encode;
use super::r#match::{is_matched, MatchedLine};
use super::request::request;
use super::response::get_url;
use super::result::line_manipulation_print;
use colored::Colorize;
use rayon::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::ops::Index;
use std::path::Path;

/*
 * purpose: call the functions from other modules to complete the task
*/
pub fn manipulate(cli_cfg: CliCfg, config: &Cfg) {
    // buffer-wrapped target file
    let md_file: BufReader<File> =
        BufReader::new(File::open(cli_cfg.filename.as_path()).expect("can not open target file"));
    let mut lines: Vec<String> = md_file.lines().map(|item| item.unwrap()).collect();

    // check the file line by line
    // for line in lines.par_iter_mut() {
    //     line.push('\n'); // append the missing newline byte

    //     if is_matched(line) {
    //         manipulate_mthed_line(line, &cli_cfg, config);
    //     }
    // }

    lines.par_iter_mut().for_each(|line| {
        line.push('\n');
        if is_matched(line) {
            manipulate_mthed_line(line, &cli_cfg, config);
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
 * embeded match is soooo ugly:(
*/
fn manipulate_mthed_line(line: &mut String, _cli_cfg: &CliCfg, config: &Cfg) {
    let mut mth: MatchedLine = MatchedLine::new(line);
    let image_path: &Path = Path::new(mth.line.index(mth.range.clone()));
    let image_name = image_path.file_name().expect("can not get image name");
    let image_path_clone = image_path.to_path_buf();

    if image_path.exists() && image_path.is_file() {
        match encode(image_path) {
            Ok(encoded_file_contents) => {
                match request(config, image_name.to_str().unwrap(), encoded_file_contents) {
                    Ok(res) => match get_url(res) {
                        Some(url) => {
                            mth.replace(url.as_str());
                            line_manipulation_print(
                                image_path_clone.as_path(),
                                "DONE".green(),
                                "Successfully uploaded!",
                            );
                        }
                        None => line_manipulation_print(
                            image_path_clone.as_path(),
                            "FAILED".red(),
                            "didn't find html_url in the response body",
                        ),
                    },
                    Err(_) => {
                        line_manipulation_print(
                            image_path_clone.as_path(),
                            "FAILED".red(),
                            "the request is not sent",
                        );
                    }
                }
            }
            Err(_) => {
                line_manipulation_print(
                    image_path_clone.as_path(),
                    "FAILED".red(),
                    "encoding file failed",
                );
            }
        }
    } else {
        line_manipulation_print(
            image_path_clone.as_path(),
            "FAILED".red(),
            "file doesn't exist or isn't a regular file",
        );
    }
}
