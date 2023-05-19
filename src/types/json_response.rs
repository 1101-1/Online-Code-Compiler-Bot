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

#[derive(Serialize, Debug)]
pub struct OtherPlayGroundRequest {
    pub code: String,
    pub codeld: Option<String>,
    pub input: String,
    pub language: String,
}

#[derive(Debug, Deserialize)]
pub struct OtherPlayGroundResponse {
    pub success: bool,
    pub errors: Vec<String>,
    pub data: Data,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Data {
    pub sourceCode: String,
    pub status: i32,
    pub errorCode: i32,
    pub error: Option<String>,
    pub outputType: i32,
    pub output: String,
    pub outputStyle: Option<String>,
    pub date: String,
    pub language: String,
    pub input: String,
    pub id: i32,
}