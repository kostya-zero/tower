use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: Option<Message>,
    pub raw_string: String,
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub content: String,
    pub username: String,
    pub timestamp: Option<String>,
    pub client: String,
    pub avatar_url: Option<String>,
}
