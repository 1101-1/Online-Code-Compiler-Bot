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
pub struct PythonPlayGroundRequest {
    pub code: String,
    pub codeld: Option<String>,
    pub input: String,
    pub language: String,
}

#[derive(Debug, Deserialize)]
pub struct PythonPlayGroundResponse {
    pub success: bool,
    pub errors: Vec<String>,
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub source_code: String,
    pub status: i32,
    pub error_code: i32,
    pub error: Option<String>,
    pub output_type: i32,
    pub output: String,
    pub output_style: Option<String>,
    pub date: String,
    pub language: String,
    pub input: String,
    pub id: i32,
}