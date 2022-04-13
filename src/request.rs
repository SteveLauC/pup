//! enerything relevant to http requests
use super::config::Config;
use anyhow::Result;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue};

/// send request
pub fn request(config: &Config, file_name: String, file_contents: String) -> Result<Response> {
    // init a client
    let client: Client = Client::new();

    // init the header
    let mut header: HeaderMap = HeaderMap::new();
    header.append("User-Agent", HeaderValue::from_static("pup"));
    header.append(
        "accept",
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    header.append("Content-Type", HeaderValue::from_static("application/json"));
    header.append(
        "Authorization",
        HeaderValue::from_str(config.token.as_str())?,
    );

    // init the json body
    let json_body: String = format!("{{\"message\": \"upload\", \"commiter\": {{\"name\": \"{}\", \"email\":\"{}\"}}, \"content\": \"{}\"}}", config.name, config.mail, file_contents);

    let url: String = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        config.name, config.repo, file_name
    );
    let res: Response = client.put(url).headers(header).body(json_body).send()?;
    Ok(res)
}
