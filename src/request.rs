//! Sends HTTP PUT request

use crate::{config::UserConfig, encode::encode};
use anyhow::Result;
use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue},
};
use std::path::Path;

/// Picture uploader.
#[derive(Debug)]
pub struct Uploader {
    client: Client,
    header: HeaderMap,
}

impl Uploader {
    /// Initialize an [`Uploader`].
    pub fn init(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.append("User-Agent", HeaderValue::from_static("pup"));
        headers.append(
            "accept",
            HeaderValue::from_static("application/vnd.github.v3+json"),
        );
        // The GitHub API requires that the token header value should be
        // "token TOKEN"
        let token_with_prefix = format!("token {}", token);
        headers.append(
            "Authorization",
            HeaderValue::from_str(token_with_prefix.as_str())
                .expect("failed to parse header value"),
        );

        Self {
            client: Client::new(),
            header: headers,
        }
    }

    /// Upload file specified in `path` to the Repo.
    pub fn upload<P: AsRef<Path>>(&self, path: P, user_cfg: &UserConfig) -> Result<Response> {
        let encoded_file_contents = encode(path.as_ref())?;
        let file_name = path.as_ref().file_name().unwrap().to_str().unwrap();

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
        let mut json_body= format!(
            "{{\"message\": \"upload\", \"commiter\": {{\"name\": \"{}\", \"email\":\"{}\"}}, \"content\": \"",
            user_cfg.github_user_name,
            user_cfg.mail
        ).into_bytes();
        json_body.extend_from_slice(&encoded_file_contents);
        json_body.extend_from_slice("\"}".as_bytes());

        // target URL
        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            user_cfg.github_user_name, user_cfg.github_repo_name, file_name
        );

        let res = self
            .client
            .put(url)
            .headers(self.header.clone())
            .body(json_body)
            .send()?;
        Ok(res)
    }
}
