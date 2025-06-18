use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ConnectionResult {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct SendResult {
    pub success: bool,
    pub message: String,
}
