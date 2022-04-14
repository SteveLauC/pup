use reqwest::blocking::Response;
use reqwest::StatusCode;
use serde_json::{from_str, Value};

pub fn get_url(body: Response) -> Option<String> {
    match body.status() {
        StatusCode::ACCEPTED => {
            let body: String = body.text().ok()?;
            let val: Value = from_str(&body).ok()?;
            if let Value::String(ref url) = val["content"]["html_url"] {
                Some(url.clone())
            } else {
                None
            }
        }
        StatusCode::NOT_FOUND => {
            // eprintln!("{:?}", body);
            eprintln!("GitHub api is invalid");
            None
        }
        StatusCode::UNPROCESSABLE_ENTITY => {
            // eprintln!("{:?}", body);
            eprintln!("Token is invalid");
            None
        }
        StatusCode::CONFLICT => {
            eprintln!("Seems the file you want upload already exists");
            None
        }
        _ => None,
    }
}
