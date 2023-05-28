use serde::Serialize;

#[derive(Serialize)]
pub struct DefaultError {
    pub message: String,
    pub error_code: String,
}

#[derive(Serialize)]
pub struct DefaultMsg {
    pub message: String,
    pub message_code: String,
}