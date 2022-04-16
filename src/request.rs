/*
 * request.rs: sends HTTP PUT request
*/

use crate::config::Cfg;
use anyhow::Result;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue};

/*
 * purpose: send PUT request to the GitHub server
 * return: None if there is anything wrong with network connection or message initialization
*/
pub fn request(config: &Cfg, file_name: &str, file_contents: Vec<u8>) -> Result<Response> {
    // init the client
    let client: Client = Client::new();

    // init the header
    let mut header: HeaderMap = HeaderMap::new();
    header.append("User-Agent", HeaderValue::from_static("pup"));
    header.append(
        "accept",
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    header.append(
        "Authorization",
        HeaderValue::from_str(config.token.as_str())?,
    );

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
    let res: Response = client.put(url).headers(header).body(json_body).send()?;
    Ok(res)
}
