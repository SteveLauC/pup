//! HTTP response handling

use reqwest::{blocking::Response, StatusCode};
use serde_json::{from_str, Value};
use thiserror::Error;

/// type to represent failed cases
#[derive(Debug, Error)]
pub enum FailedCases {
    #[error("409: Conflict")]
    Conflict,
    #[error("422: ValidationFailed")]
    ValidationFailed,
    #[error("The picture is uploaded but no URL is returned.")]
    CreatedButUrlNotFound,
    #[error("This error is not covered by pup")]
    NotCoveredCase,
}

/// Parse URL from the returned json body
pub fn get_url(body: Response) -> Result<String, FailedCases> {
    match body.status() {
        // returns 201
        StatusCode::CREATED => {
            let body: String = body.text().expect("can not get response body");
            let val: Value = from_str(&body).expect("can not parse response body");
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
        _ => {
            eprintln!("Response: {:#?}", body);
            Err(FailedCases::NotCoveredCase)
        }
    }
}
