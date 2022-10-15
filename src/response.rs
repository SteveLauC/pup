//! HTTP response handling

use reqwest::{blocking::Response, StatusCode};
use serde_json::{from_str, Value};
use std::{error::Error, fmt};

/// type to represent failed cases
#[derive(Debug)]
pub enum FailedCases {
    Conflict,
    ValidationFailed,
    CreatedButUrlNotFound,
    NotCoveredCase,
}

// Implement Display and Debug for our error type
// so that we can have std::error::Error implmented
impl fmt::Display for FailedCases {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for FailedCases {}

/// Parse URL from the returned json body
pub fn get_url(body: Response) -> Result<String, FailedCases> {
    match body.status() {
        // returns 201
        StatusCode::CREATED => {
            let body: String = body.text().expect("can not get response body");
            let val: Value =
                from_str(&body).expect("can not parse response body");
            if let Value::String(ref url) = val["content"]["html_url"] {
                Ok(url.clone())
            } else {
                Err(FailedCases::CreatedButUrlNotFound)
            }
        }
        // returns 409
        StatusCode::CONFLICT => Err(FailedCases::Conflict),
        // returns 422
        StatusCode::UNPROCESSABLE_ENTITY => Err(FailedCases::ValidationFailed),
        // other cases of failure which are not covered right now
        _ => Err(FailedCases::NotCoveredCase),
    }
}
