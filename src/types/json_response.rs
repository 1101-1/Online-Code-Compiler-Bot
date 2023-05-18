use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct RustPlayGroundRequest {
    pub code: String,
    pub version: String,
    pub optimize: String,
    pub test: bool,
    pub separate_output: bool,
    pub color: bool,
    pub backtrace: String,
}

#[derive(Deserialize, Debug)]
pub struct RustPlayGroundResponse {
    pub result: Option<String>,
    pub error: Option<String>,
}
