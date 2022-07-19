use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BadRequest {
    pub error_codes: Vec<u64>,
}

#[derive(Debug)]
pub struct Foo {
    pub error_code: Option<u64>,
    pub message: &'static str,
    pub r#type: &'static str,
}
