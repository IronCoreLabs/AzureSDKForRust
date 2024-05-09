use quick_error::quick_error;
use serde::{Deserialize, Serialize};
use serde_json;
use std;
use std::num;
use std::str;
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorResponse {
    #[error("Unrecognized Azure error response:\n{error_description}\n")]
    GenericError { error_description: String },
}

quick_error! {
    #[derive(Debug)]
    pub enum AzureError {
        JSONError(err: serde_json::Error) {
            from()
            display("json error: {}", err)
            source(err)
        }
        ParseIntError(err: num::ParseIntError){
            from()
            display("Parse int error: {}", err)
            source(err)
        }
        GenericErrorWithText(err: String) {
            display("Generic error: {}", err)
        }
    }
}
