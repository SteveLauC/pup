use super::config::Cfg;
use super::request::request;
use super::response::get_url;
use anyhow::Result;
use base64::{encode_config_slice, STANDARD};
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::ops::Index;
use std::path::Path;

use super::r#match::{is_matched, MatchedLine};

pub fn manipulate(target: &Path, config: &Cfg) -> Result<()> {
    let md_file: BufReader<File> = BufReader::new(File::open(target)?);
    let mut lines: Vec<String> = md_file.lines().map(|item| item.unwrap()).collect();

    for line in lines.iter_mut() {
        line.push('\n');

        if is_matched(line) {
            let mut mth: MatchedLine = MatchedLine::new(line);
            let image_path: &Path = Path::new(mth.line.index(mth.range.clone()));

            if !image_path.exists() {
                eprintln!("{:?} doesn't exist.", image_path);
                continue;
            }

            if image_path.is_file() {
                let mut file_contents: Vec<u8> = Vec::new();
                let file_name: &OsStr = image_path.file_name().expect("can not get file name");
                let mut image_file: File = File::open(image_path)?;

                image_file.read_to_end(&mut file_contents)?;

                let mut contents: Vec<u8> = Vec::new();
                contents.resize(file_contents.len() * 4 / 3 + 4, 0);
                let n_bytes: usize = encode_config_slice(file_contents, STANDARD, &mut contents);
                contents.truncate(n_bytes); // remove the trailing zeros

                let res = request(config, file_name.to_str().unwrap(), contents)?;

                match get_url(res) {
                    Some(url) => {
                        mth.replace(url.as_str());
                    }
                    None => {
                        continue;
                    }
                }
            }
        }
    }

    let mut md_file = OpenOptions::new().write(true).truncate(true).open(target)?;
    for line in lines.iter() {
        md_file.write(line.as_bytes())?;
    }
    Ok(())
}
