//! request.rs: sends HTTP PUT request

use crate::config::Cfg;
use anyhow::Result;
use reqwest::{
    blocking::{Client, Response},
    header::HeaderMap,
};

/// purpose: send PUT request to the GitHub server
///
/// arguments:
///     * `config`: user configuration
///     * `file_name`: name of the image file
///     * `file_contents`: base64 encoded file contents
///
/// return: None if there is anything wrong with network connection or message initialization
pub fn request(
    client: &Client,
    header: &HeaderMap,
    config: &Cfg,
    file_name: &str,
    file_contents: Vec<u8>,
) -> Result<Response> {
    // init the json body
    /*
    {
        "message": "upload",
        "commiter": {
            "name": "commiter-name",
            "email": "commiter-email"
        },
        "content": "file-contents"
    }
    */
    let mut json_body: Vec<u8>= format!(
        "{{\"message\": \"upload\", \"commiter\": {{\"name\": \"{}\", \"email\":\"{}\"}}, \"content\": \"", 
        config.name,
        config.mail
    ).as_bytes().to_vec();
    json_body.extend_from_slice(&file_contents);
    json_body.extend_from_slice("\"}".as_bytes());

    // target URL
    let url: String = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        config.name, config.repo, file_name
    );

    let res: Response = client
        .put(url)
        .headers(header.clone())
        .body(json_body)
        .send()?;
    Ok(res)
}
