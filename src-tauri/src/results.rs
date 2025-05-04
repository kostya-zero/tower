use serde::Serialize;


#[derive(Serialize, Debug)]
pub struct ConnectionResult {
    pub success: bool,
    pub message: String,
}