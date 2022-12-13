//! Process the target markdown file, check line by line if there is a markdown
//! image link, send the request and replace the path

use crate::{
    cli::CliCfg,
    config::Cfg,
    encode::encode,
    r#match::MatchedLine,
    request::{client_and_header, request},
    response::get_url,
    result::MdManipulationResult,
};
use arboard::Clipboard;
use colored::Colorize;
use rayon::prelude::*;
use reqwest::{blocking::Client, header::HeaderMap};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    ops::Index,
    path::Path,
    sync::{Arc, Mutex},
};

/// Call the functions from other modules to complete the task
pub fn md_manipulate(
    cli_cfg: &CliCfg,
    config: &Cfg,
    r: Arc<Mutex<MdManipulationResult>>,
) {
    // buffer-wrapped target file
    let md_file: BufReader<File> = BufReader::new(
        File::open(cli_cfg.file_path.as_path())
            .expect("can not open target file"),
    );
    let mut lines: Vec<String> =
        md_file.lines().map(|item| item.unwrap()).collect();

    let (client, headers) = client_and_header(config);

    lines.par_iter_mut().for_each(|line| {
        line.push('\n');

        if let Some(mut mth) = MatchedLine::new(line) {
            // to escape simultaneous occurrence of mutable and immutable borrowing
            let image_path =
                Path::new(mth.line.index(mth.range.clone())).to_path_buf();
            r.lock().unwrap().res_handling(
                manipulate_mthed_line(&client, &headers, &mut mth, config),
                image_path.as_path(),
            );
        }
    });

    // write to the target markdown file
    let mut md_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(cli_cfg.file_path.as_path())
        .expect("can not open target markdown file for writing purpose");

    lines.iter().for_each(|line| {
        md_file
            .write(line.as_bytes())
            .expect("can not write to the target markdown file");
    });

    // print the statistical result
    println!("{}", r.lock().unwrap());
}

/// Deal with every matched line.
///
/// This is the helper function used inside of [`md_manipulate`].
fn manipulate_mthed_line<'a>(
    client: &Client,
    headers: &HeaderMap,
    mth: &'a mut MatchedLine<'a>,
    config: &Cfg,
) -> Result<(), Box<dyn std::error::Error>> {
    let image_path = Path::new(mth.line.index(mth.range.clone()));
    let image_name = image_path.file_name().expect("can not get image name");
    let encoded_file_contents = encode(image_path)?;

    let res = request(
        client,
        headers,
        config,
        image_name.to_str().unwrap(),
        encoded_file_contents,
    )?;
    let url = get_url(res)?;
    mth.replace(url.as_str());
    Ok(())
}

/// Manipulate single image file
pub fn img_manipulate(cli_cfg: &CliCfg, config: &Cfg) {
    let (client, headers) = client_and_header(config);
    let encoded_file_contents = encode(cli_cfg.file_path.as_path())
        .expect("failed to encode img contents in base64 format");
    let res = request(
        &client,
        &headers,
        config,
        cli_cfg.file_path.file_name().unwrap().to_str().unwrap(),
        encoded_file_contents,
    )
    .expect("failed to send PUT request");
    let url =
        get_url(res).expect("failed to extract URL from the response message");
    let mut clipboard =
        Clipboard::new().expect("failed to initialize a clipboard instance");
    clipboard
        .set_text(url.as_str())
        .expect("can not store the returned URL to clipboard");

    println!("{} [{}]", cli_cfg.file_path.display(), "done".green());
    println!(
        "The returned URL is: {} (stored to your clipboard)",
        url.as_str().purple()
    );
}
