use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BadRequest {
    pub error_description: String,
}

#[derive(Debug)]
pub struct Foo {
    pub code: &'static str,
    pub message: &'static str,
    pub r#type: &'static str,
}
