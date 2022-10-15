//! Sends HTTP PUT request

use crate::config::Cfg;
use anyhow::Result;
use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue},
};

/// Initialize a HTTP client and header
pub fn client_and_header(config: &Cfg) -> (Client, HeaderMap) {
    // init the client
    let http_client = Client::new();
    // init the header
    let mut headers: HeaderMap = HeaderMap::new();
    headers.append("User-Agent", HeaderValue::from_static("pup"));
    headers.append(
        "accept",
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.append(
        "Authorization",
        HeaderValue::from_bytes(config.token.as_bytes())
            .expect("failed to parse header value"),
    );

    (http_client, headers)
}

/// Send PUT request to the GitHub server
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
