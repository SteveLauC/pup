use reqwest::blocking::Response;
use reqwest::StatusCode;
use serde_json::{from_str, Value};

// TODO: add user-friendly output
pub fn get_url(body: Response) -> Option<String> {
    match body.status() {
        StatusCode::CREATED => {
            let body: String = body.text().ok()?;
            // DEBUG
            // println!("{}", body);

            // DEBUG
            let val: Value = from_str(&body).ok()?;

            // DEBUG
            // println!("{}", val);
            // DEBUG

            if let Value::String(ref url) = val["content"]["html_url"] {
                Some(url.clone())
            } else {
                None
            }
        }
        _ => None,
    }
}
