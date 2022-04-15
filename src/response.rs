/*
 * response.rs:
*/
use reqwest::blocking::Response;
use reqwest::StatusCode;
use serde_json::{from_str, Value};

/*
 * purpose: parse URL from the returned json body
*/
pub fn get_url(body: Response) -> Option<String> {
    match body.status() {
        StatusCode::CREATED => {
            let body: String = body.text().ok()?;
            let val: Value = from_str(&body).ok()?;
            if let Value::String(ref url) = val["content"]["html_url"] {
                Some(url.clone())
            } else {
                None
            }
        }
        _ => None,
    }
}
